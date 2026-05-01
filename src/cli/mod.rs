use clap::{builder::TypedValueParser, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "FILE")]
    pub path: Option<PathBuf>,
    #[arg(short, long, help = "Prompt for mnemonic seed phrase from which to derive the wallet.")]
    pub phrase: bool,
    #[arg(short, long, help = "Mnemonic seed phrase word count.", value_name = "num_words", value_parser = clap::builder::PossibleValuesParser::new(["12", "24"]).map(|s| s.parse::<u8>().unwrap()))]
    pub words: Option<u8>,
    #[arg(short, long, help = "The number of addresses to generate", value_name = "num_addresses", value_parser = clap::value_parser!(u32))]
    pub addresses: Option<u32>,
    #[arg(short, long, help = "Address index offset from which to begin generating addresses", value_name = "offset", value_parser = clap::value_parser!(u32))]
    pub offset: Option<u32>,
    #[arg(short, long, help = "The number of address indicies to skip between derivations.", value_name = "skip", value_parser = clap::value_parser!(usize), conflicts_with = "random")]
    pub skip: Option<usize>,
    #[arg(short, long, help = "Randomize address indicies. When this is set, -s is ignored, and -o and -m are used to define the range from which a random address value is chosen.")]
    pub random: bool,
    #[arg(short = 'm', long = "max", help = "Maximum address index height, from offset. Overridden by -a if smaller than that value.", value_name = "max_height", value_parser = clap::value_parser!(u32), requires = "random")]
    pub height: Option<u32>,
    #[arg(short, long, help = "Don't print wallet details to stdout.")]
    pub quiet: bool,
    #[arg(short, long, help = "Export mnemonic seed phrase to wallet.", value_name = "app_name", num_args = 0.., value_parser = clap::builder::PossibleValuesParser::new(["sage", "chia"]))]
    pub export: Option<Vec<String>>,
    #[arg(long, help = "Export the wallet mnemonic/private key. Without this flag, it will export the wallet public key.", requires = "export")]
    pub export_hot: bool,
    #[arg(long, help = "Enable named wallet export. Will generate a name from the fingerprint.")]
    pub enable_naming: bool,
}

pub mod input;
