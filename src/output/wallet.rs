use chia::bls::DerivableKey;
use chia::puzzles::{standard::StandardArgs, DeriveSynthetic};
use chia::protocol::Bytes32;

use crate::derive::Wallet;
use crate::output;

pub fn print_wallet(wallet: &Wallet, label: &str) {
    println!(
        "\nLabel: {}\nFingerprint: {}\nMnemonic: {}\nMaster Private Key: {}\nFarmer Private Key: {}\nMaster Public Key: {}\nFarmer Public Key: {}\nPool Public Key: {}\nWallet Obeserver Key: {}\n",
        label,
        wallet.fingerprint,
        wallet.mnemonic.to_string(),
        hex::encode(wallet.master_secret_key.to_bytes()),
        hex::encode(wallet.farmer_secret_key.to_bytes()),
        hex::encode(wallet.master_public_key.to_bytes()),
        hex::encode(wallet.farmer_public_key.to_bytes()),
        hex::encode(wallet.pool_public_key.to_bytes()),
        hex::encode(wallet.unhardened_intermediate.to_bytes()),
    );

    let c1w = output::compute_col_width(&wallet.indices);
    println!(output::ROW_FMT!(),
        "ID","Type","Address","Public Key",
        col_1_width=&c1w,
    );
    for i in wallet.indices.iter() {
        let hsyn = wallet.hardened_intermediate.derive_hardened(*i).derive_synthetic().public_key();
        let hhash: Bytes32 = StandardArgs::curry_tree_hash(hsyn).into();
        let haddr = output::encode_address(hhash);
        println!(
            output::ROW_FMT!(),
            i, "Hardened", haddr, hex::encode(hsyn.to_bytes()),
            col_1_width=&c1w,
        );
    }
    for i in wallet.indices.iter() {
        let usyn = wallet.unhardened_intermediate.derive_unhardened(*i).derive_synthetic();
        let uhash: Bytes32 = StandardArgs::curry_tree_hash(usyn).into();
        let uaddr = output::encode_address(uhash);
        println!(
            output::ROW_FMT!(),
            i, "Unhardened", uaddr, hex::encode(usyn.to_bytes()),
            col_1_width=&c1w,
        );
    }
}
