// Copyright 2025 Abraham Sewill <abraham.sewill@proton.me>
// SPDX-License-Identifier: MIT

use clap::Parser;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use xch_keygen::cli;
use xch_keygen::cli::Args;
use xch_keygen::derive;
use xch_keygen::export;
use xch_keygen::output;

fn main() {
    let args = Args::parse();
    let mut rng = ChaCha20Rng::from_os_rng();

    let mnemonic = cli::input::read_mnemonic(&args, &mut rng);
    let wallet = derive::Wallet::new(&mnemonic, &args, &mut rng);

    let naming = args.enable_naming;
    let quiet = args.quiet;
    let label = if naming {
        derive::fingerprint_name(wallet.fp)
    } else {
        String::new()
    };

    let exports = args.export.unwrap_or_default();
    export::run_exports(&wallet, &exports, args.export_hot, &label);

    if quiet {
        return;
    }
    output::print_wallet(&wallet, &label);
}
