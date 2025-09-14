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
}

use std::{
    io::{self, prelude::*, IsTerminal, Write},
    fs::File,
    path::PathBuf
};
use bip39::Mnemonic;
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
    let fpk = msk.derive_hardened(12381_u32).derive_hardened(8444).derive_hardened(0).derive_hardened(0).public_key();
    let ppk = msk.derive_hardened(12381_u32).derive_hardened(8444).derive_hardened(1).derive_hardened(0).public_key();

    // Print wallet details.
    println!(
        "\nFingerprint: {}\nMnemonic: {}\nMaster Public Key: {}\nFarmer Public Key: {}\nPool Public Key: {}\nWallet Obeserver Key: {}\n",
        fp.to_string(),
        mnemonic.to_string(),
        hex::encode(mpk.to_bytes()),
        hex::encode(fpk.to_bytes()),
        hex::encode(ppk.to_bytes()),
        hex::encode(ui.to_bytes()),
    );

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