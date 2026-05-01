use chia::bls::DerivableKey;
use chia::puzzles::{standard::StandardArgs, DeriveSynthetic};
use chia::protocol::Bytes32;

use crate::chia_rpc::daemon::add_key::Command;
use crate::chia_rpc::websocket::Request;
use crate::derive::Wallet;
use crate::output;

pub fn print_wallet(wallet: &Wallet, label: &str) {
    println!(
        "\nLabel: {}\nFingerprint: {}\nMnemonic: {}\nMaster Private Key: {}\nFarmer Private Key: {}\nMaster Public Key: {}\nFarmer Public Key: {}\nPool Public Key: {}\nWallet Obeserver Key: {}\n",
        label,
        wallet.fp,
        wallet.mnemonic.to_string(),
        hex::encode(wallet.msk.to_bytes()),
        hex::encode(wallet.fsk.to_bytes()),
        hex::encode(wallet.mpk.to_bytes()),
        hex::encode(wallet.fpk.to_bytes()),
        hex::encode(wallet.ppk.to_bytes()),
        hex::encode(wallet.ui.to_bytes()),
    );

    let c1w = output::compute_col_width(&wallet.indices);
    println!(output::ROW_FMT!(),
        "ID","Type","Address","Public Key",
        col_1_width=&c1w,
    );
    for i in wallet.indices.iter() {
        let hsyn = wallet.hi.derive_hardened(*i).derive_synthetic().public_key();
        let hhash: Bytes32 = StandardArgs::curry_tree_hash(hsyn).into();
        let haddr = output::encode_address(hhash);
        println!(
            output::ROW_FMT!(),
            i, "Hardened", haddr, hex::encode(hsyn.to_bytes()),
            col_1_width=&c1w,
        );
    }
    for i in wallet.indices.iter() {
        let usyn = wallet.ui.derive_unhardened(*i).derive_synthetic();
        let uhash: Bytes32 = StandardArgs::curry_tree_hash(usyn).into();
        let uaddr = output::encode_address(uhash);
        println!(
            output::ROW_FMT!(),
            i, "Unhardened", uaddr, hex::encode(usyn.to_bytes()),
            col_1_width=&c1w,
        );
    }
}

pub fn build_add_key_command(wallet: &Wallet, export_hot: bool, label: &str) -> Command {
    Command {
        kc_service: None,
        kc_user: None,
        mnemonic_or_pk: if export_hot { wallet.mnemonic.to_string() } else { hex::encode(wallet.mpk.to_bytes()) },
        label: if label.is_empty() { None } else { label.to_string().into() },
        private: export_hot.into(),
    }
}

pub fn build_websocket_request(cmd: Command) -> Request<Command> {
    Request {
        ack: false,
        command: "add_key".to_string(),
        request_id: "".to_string().into(),
        origin: Some("xch-keygen".to_string()),
        destination: "daemon".to_string(),
        data: cmd,
    }
}
