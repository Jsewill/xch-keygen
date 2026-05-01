use bip39::Mnemonic;
use rand_chacha::ChaCha20Rng;
use std::fs::File;
use std::io::{self, IsTerminal, Read, Write};

use crate::cli::Args;

pub fn read_mnemonic(args: &Args, rng: &mut ChaCha20Rng) -> Mnemonic {
    let words = args.words.unwrap_or(24);
    let mut mnemonic = crate::derive::generate_mnemonic(words, rng);
    let mut input = String::new();

    if let Some(ref path) = args.path {
        if let Ok(mut file) = File::open(path) {
            let _ = file.read_to_string(&mut input);
        }
    }

    if input.is_empty() {
        if args.phrase {
            print!("Enter mnemonic seed phrase (return to skip): ");
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut input);
        }
        if !io::stdin().is_terminal() && input.is_empty() {
            let _ = io::stdin().read_line(&mut input);
        }
    }

    let trimmed = input.trim();
    if !trimmed.is_empty() {
        let lower = trimmed.to_lowercase();
        match Mnemonic::parse(&lower) {
            Ok(m) => mnemonic = m,
            Err(e) => {
                eprintln!("Couldn't parse mnemonic phrase: {e}");
                std::process::exit(1);
            }
        }
    }

    mnemonic
}
