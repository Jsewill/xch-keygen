// Copyright 2025 Abraham Sewill <abraham.sewill@proton.me>
// SPDX-License-Identifier: MIT

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Mnemonic seed word count.", value_name = "num_words", value_parser = clap::builder::PossibleValuesParser::new(["12", "24"]))]
    words: Option<String>,
    #[arg(short, long, help = "The number of addresses to generate", value_name = "num_addresses", value_parser = clap::value_parser!(u32))]
    addresses: Option<u32>,
    #[arg(short, long, help = "Index offset from which to begin generating addresses", value_name = "offset", value_parser = clap::value_parser!(u32))]
    offset: Option<u32>,
}

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
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

// Generates a Mnemonic from entropy.
pub fn generate_mnemonic(words: u8) -> Mnemonic {
    let mut rng = ChaCha20Rng::from_os_rng();
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
    let words: u8 = args.words.unwrap_or("24".to_string()).parse().unwrap_or(24).into();
    let addresses: u32 = args.addresses.unwrap_or(10);
    let offset: u32 = args.offset.unwrap_or(0);

    // Generate wallet.
    let mnemonic = generate_mnemonic(words);
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
    // Print hardened addresses.
    for i in offset..offset+addresses {
        let hsyn = hi.derive_hardened(i).derive_synthetic().public_key();
        let hhash: Bytes32 = StandardArgs::curry_tree_hash(hsyn).into();
        let haddr = encode_address(hhash);
        println!(
            "Hardened Address {}: {}",
            i, haddr
        );
    }
    // Print unhardened addresses.
    for i in offset..offset+addresses {
        let usyn = ui.derive_unhardened(i).derive_synthetic();
        let uhash: Bytes32 = StandardArgs::curry_tree_hash(usyn).into();
        let uaddr = encode_address(uhash);
        println!(
            "Address {}: {}",
            i, uaddr
        );
    }
}