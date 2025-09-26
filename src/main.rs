// Copyright 2025 Abraham Sewill <abraham.sewill@proton.me>
// SPDX-License-Identifier: MIT

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name = "FILE")]
    path: Option<PathBuf>,
    #[arg(short, long, help = "Prompt for mnemonic seed phrase from which to derive the wallet.")]
    phrase: bool,
    #[arg(short, long, help = "Mnemonic seed phrase word count.", value_name = "num_words", value_parser = clap::builder::PossibleValuesParser::new(["12", "24"]))]
    words: Option<String>,
    #[arg(short, long, help = "The number of addresses to generate", value_name = "num_addresses", value_parser = clap::value_parser!(u32))]
    addresses: Option<u32>,
    #[arg(short, long, help = "Address index offset from which to begin generating addresses", value_name = "offset", value_parser = clap::value_parser!(u32))]
    offset: Option<u32>,
    #[arg(short, long, help = "The number of address indicies to skip between derivations.", value_name = "skip", value_parser = clap::value_parser!(usize), conflicts_with = "random")]
    skip: Option<usize>,
    #[arg(short, long, help = "Randomize address indicies. When this is set, -s is ignored, and -o and -m are used to define the range from which a random address value is chosen.")]
    random: bool,
    #[arg(short = 'm', long = "max", help = "Maximum address index height, from offset. Overridden by -a if smaller than that value.", value_name = "max_height", value_parser = clap::value_parser!(u32), requires = "random")]
    height: Option<u32>,
    #[arg(short, long, help = "Don't print wallet details to stdout.")]
    quiet: bool,
    #[arg(short, long, help = "Export mnemonic seed phrase to wallet.",value_name = "app_name", num_args = 0.., value_parser = clap::builder::PossibleValuesParser::new(["sage", "chia"]))]
    export: Option<Vec<String>>,
    #[arg(long, help = "Export the wallet mnemonic/private key. Without this flag, it will export the wallet public key.", requires = "export")]
    export_hot: bool,
    #[arg(long, help = "Enable named wallet export. Will generate a name from the fingerprint.")]
    enable_naming: bool,
}

use std::{
    fs::File, io::{self, prelude::*, IsTerminal, Write}, net::TcpStream, path::PathBuf, str::FromStr
};
use dirs;
use serde_json;
use tokio;
use bip39::{Language, Mnemonic};
use chia::{
    bls::{
        master_to_wallet_hardened_intermediate, master_to_wallet_unhardened_intermediate,
        DerivableKey, SecretKey,
    },
    puzzles::{standard::StandardArgs, DeriveSynthetic},
    protocol::Bytes32,
};
use bech32::{Bech32m, Hrp};
use rand::{prelude::*, /*Rng,*/ RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sage_api::{ImportKey as SageImportKey, ImportKeyResponse as SageImportKeyResponse};
use sage_client::{self, SageRpcError};
use native_tls::{Identity, TlsConnector};
use tungstenite::{self, http::Uri, stream::MaybeTlsStream};
use xch_keygen::chia_rpc;

// Generates a Mnemonic from entropy.
pub fn generate_mnemonic(words: u8, rng: &mut ChaCha20Rng) -> Mnemonic {
    let mut entropy: [u8; 32] = [0; 32];
    rng.fill_bytes(&mut entropy);
    let mnemonic = match words {
        12 => Mnemonic::from_entropy(&entropy[..16]).expect("Could not generate mnemonic from entropy"),
        _ => Mnemonic::from_entropy(&entropy).expect("Could not generate mnemonic from entropy"),
    };
    return mnemonic
}

// Encodes a puzzle hash to an xch address.
pub fn encode_address(puzzle_hash: Bytes32) -> String {
    let hrp = Hrp::parse("xch").expect("valid hrp");
    return bech32::encode::<Bech32m>(hrp, &puzzle_hash).expect("Could not encode puzzle hash to bech32m");
}

fn main() {
    let args = Args::parse();
    let path: PathBuf = args.path.unwrap_or(PathBuf::new());
    let phrase: bool = args.phrase;
    let words: u8 = args.words.unwrap_or("24".to_string()).parse().unwrap_or(24).into();
    let addresses: u32 = args.addresses.unwrap_or(10);
    let offset: u32 = args.offset.unwrap_or(0);
    let skip: usize = args.skip.unwrap_or(0);
    let random: bool = args.random;
    let mut height: u32 = args.height.unwrap_or(offset+addresses);
    let quiet: bool = args.quiet;
    let exports: Vec<String> = args.export.unwrap_or(vec![]);
    let export_hot: bool = args.export_hot;
    let naming: bool = args.enable_naming;

    if height < offset+addresses {
        height = offset+addresses;
    }

    let mut rng = ChaCha20Rng::from_os_rng();

    // Generate wallet.
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    // Generate mnemonic phrase.
    let mut mnemonic: Mnemonic = generate_mnemonic(words, &mut rng);

    // Read from input file; whether file descriptor, or final parameter.
    let file_result = File::open(path);
    if file_result.is_ok() {
        let mut file = file_result.unwrap();
        file.read_to_string(&mut input).unwrap();
    }
    if input.is_empty() {
        if phrase {
            // Read phrase from stdin.
            print!("Enter mnemonic seed phrase (return to skip): ");
            stdout.flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
        }
        if !stdin.is_terminal() {
            // Check for input and get phrase.
            io::stdin().read_line(&mut input).unwrap();
        }
    }
    if !input.is_empty() {
        input = input.to_lowercase();
        let split = input.split_whitespace();
        let sc = split.count();
        // If supplied string word count is supported, attempt to parse it.
        if sc == 12 || sc == 24 {
            mnemonic = Mnemonic::parse(&input).expect("Couldn't parse mnemonic phrase.");
        }
    }
    // Derive wallet.
    let msk = SecretKey::from_seed(&mnemonic.to_seed(""));
    let hi = master_to_wallet_hardened_intermediate(&msk);
    let mpk = msk.public_key();
    let ui = master_to_wallet_unhardened_intermediate(&mpk);
    let fp = mpk.get_fingerprint();
    let fsk = msk.derive_hardened(12381_u32).derive_hardened(8444).derive_hardened(0).derive_hardened(0);
    let fpk = fsk.public_key();
    let ppk = msk.derive_hardened(12381_u32).derive_hardened(8444).derive_hardened(1).derive_hardened(0).public_key();

    // Get start and end.
    let mut indices: Vec<u32>;
    if !random {
        indices = (offset..offset+addresses*(skip as u32+1)).step_by(skip+1).collect();
    } else {
        indices = (offset..height).collect();
        indices.shuffle(&mut rng);
        indices = indices.into_iter().take(addresses as usize).collect();
        indices.sort(); 
    }

    // Generate wallet name from fingerprint.
    let mut fphrase: String = String::new();
    if naming {
        let wl = Language::default().word_list();
        let mut fwords: Vec<&str> = Vec::new();
        for i in (0..3).rev() {
            fwords.push(wl[(fp>>i*11&0x7ff) as usize]);
        }
        fphrase = fwords.join("-").to_string();
    }

    // Export to specified wallet application(s).
    let label: &str = fphrase.as_str();
    for export in exports {
        let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
        match export.as_str() {
            "sage" => {
                let sage_rpc_client = sage_client::Client::new().expect("Failed to set up Sage RPC client.");
                let ikreq: SageImportKey = SageImportKey {
                    name: label.to_string(),
                    key: if export_hot { mnemonic.to_string() } else { hex::encode(mpk.to_bytes()) },
                    derivation_index: *indices.last().unwrap_or(&addresses),
                    emoji: Some("".to_string()),
                    save_secrets: true,
                    login: false,
                };
                runtime.block_on(async {
                    let _resp: Result<SageImportKeyResponse, SageRpcError> =  sage_rpc_client.import_key(ikreq).await;
                });
            },
            "chia" => {
                // Get SSL Certificates.
                let mut crtbuf = Vec::new();
                let mut keybuf = Vec::new();
                let homepbuf: PathBuf = dirs::home_dir().expect("Couldn't get path to the user's home directory.");
                let nodepbuf: PathBuf = homepbuf.join(".chia/mainnet/config/ssl/daemon");
                let crtpbuf: PathBuf = nodepbuf.join("private_daemon.crt");
                let keypbuf: PathBuf = nodepbuf.join("private_daemon.key");
                File::open(crtpbuf).expect("Couldn't get chia SSL certificates (.crt).")
                    .read_to_end(&mut crtbuf).expect("Couldn't read from the chia SSL certificate (.crt) file.");
                File::open(keypbuf).expect("Couldn't get chia SSL certificates (.key).")
                    .read_to_end(&mut keybuf).expect("Couldn't read from the chia SSL certificate (.key) file.");
                
                // Build identity and prepare websocket connection.
                let ident = Identity::from_pkcs8(&crtbuf, &keybuf).expect("Couldn't produce an identity from the chia SSL certificate pair.");
                let tlsconn = TlsConnector::builder()
                    .identity(ident)
                    .danger_accept_invalid_certs(true)
                    .build()
                    .expect("Failed to build chia rpc websocket request");
                let uri = Uri::from_str("wss://localhost:55400/").unwrap();
                // Make the connections
                let tcps = TcpStream::connect(format!("{}:{}", uri.host().unwrap(), uri.port().unwrap())).expect("Couldn't establish a TCP connection with the chia daemon. Make sure the chia daemon is started.");
                let tlss = tlsconn.connect(uri.host().unwrap(), tcps).expect("Couldn't establish TLS connection with the chia daemon. Make sure the daemon ssl certificates are present in the chia data directory.");
                let tlss = MaybeTlsStream::NativeTls(tlss);
                // Make the handshake.
                let req = tungstenite::handshake::client::Request::builder()
                    .uri(uri.to_string())
                    .header("Host", uri.host().unwrap())
                    .header("Upgrade", "websocket")
                    .header("Connection", "Upgrade")
                    .header("Sec-WebSocket-Key", tungstenite::handshake::client::generate_key())
                    .header("Sec-WebSocket-Version", "13")
                    .body(()).expect("Couldn't build websocket handshake request for chia daemon.");
                let (mut ws_stream, _resp) = tungstenite::client(req, tlss).expect("Couldn't complete websocket handshake with chia daemon.");
                // Build the request.
                let cmddata = chia_rpc::daemon::add_key::Command{
                    kc_service:None,
                    kc_user:None,
                    mnemonic_or_pk: if export_hot { mnemonic.to_string() } else { hex::encode(mpk.to_bytes()) },
                    label: if label.is_empty() { None } else { label.to_string().into() },
                    private: export_hot.into(),
                };
                let reqdata = chia_rpc::websocket::Request{
                    command:"add_key".to_string(),
                    ack: false,
                    request_id: "".to_string().into(), // Generate this if necessary.
                    origin: Some("xch-keygen".to_string()),
                    destination: "daemon".to_string(),
                    data: cmddata,
                };
                let reqjson = serde_json::to_string(&reqdata).expect("Couldn't serialize chia daemon websocket request.");
                // Send the request.
                ws_stream.send(tungstenite::Message::Text(reqjson.into())).expect("Couldn't make the websocket request to the chia daemon.");

                // Get the response.
                if let Ok(msg) = ws_stream.read() {
                    let respjson: chia_rpc::daemon::add_key::Response = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                    if !respjson.data.success {
                        eprintln!("Export to chia was not successful. Error: {}", respjson.data.error.unwrap_or("No error was provided.".to_string()));
                    }
                }
            }
            _ => {
                if !export.is_empty() {
                    panic!("{} RPC is not yet supported.", export);
                }
            },
        }
    }

    if quiet {
        return;
    }
    // Print wallet details.
    println!(
        "\nLabel: {}\nFingerprint: {}\nMnemonic: {}\nMaster Private Key: {}\nFarmer Private Key: {}\nMaster Public Key: {}\nFarmer Public Key: {}\nPool Public Key: {}\nWallet Obeserver Key: {}\n",
        fphrase.to_string(),
        fp.to_string(),
        mnemonic.to_string(),
        hex::encode(msk.to_bytes()),
        hex::encode(fsk.to_bytes()),
        hex::encode(mpk.to_bytes()),
        hex::encode(fpk.to_bytes()),
        hex::encode(ppk.to_bytes()),
        hex::encode(ui.to_bytes()),
    );

    // Print hardened addresses.
    for i in indices.iter() {
        let hsyn = hi.derive_hardened(*i).derive_synthetic().public_key();
        let hhash: Bytes32 = StandardArgs::curry_tree_hash(hsyn).into();
        let haddr = encode_address(hhash);
        println!(
            "Hardened Address {}: {}",
            i, haddr
        );
    }
    // Print unhardened addresses.
    for i in indices.iter() {
        let usyn = ui.derive_unhardened(*i).derive_synthetic();
        let uhash: Bytes32 = StandardArgs::curry_tree_hash(usyn).into();
        let uaddr = encode_address(uhash);
        println!(
            "Address {}: {}",
            i, uaddr
        );
    }
}