use bech32::{Bech32m, Hrp};
use chia::protocol::Bytes32;
use std::cmp;

pub fn encode_address(puzzle_hash: Bytes32) -> String {
    let hrp = Hrp::parse("xch").expect("valid hrp");
    bech32::encode::<Bech32m>(hrp, &puzzle_hash).expect("Could not encode puzzle hash to bech32m")
}

pub fn compute_col_width(indices: &[u32]) -> usize {
    let mut c1w: u32 = 1000;
    c1w = indices.last().unwrap_or(&c1w).checked_ilog10().unwrap_or(2) + 1;
    (cmp::max(c1w, 3) as usize).try_into().unwrap_or(4)
}

macro_rules! ROW_FMT {
    () => { "{:<col_1_width$} {:<11} {:<63} {:<97}" };
}

pub(crate) use ROW_FMT;

mod wallet;

pub use wallet::print_wallet;
