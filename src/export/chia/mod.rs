//! Chia daemon export via SSL-protected websocket.
//!
//! This module handles the transport layer: loading SSL certificates,
//! establishing TLS connections, and constructing `add_key` RPC payloads.

use crate::chia_rpc::daemon::add_key::Command;
use crate::chia_rpc::websocket::Request;
use crate::derive::Wallet;

mod ssl;
mod websocket;

pub fn build_add_key_command(wallet: &Wallet, export_hot: bool, label: &str) -> Command {
    Command {
        kc_service: None,
        kc_user: None,
        mnemonic_or_pk: if export_hot { wallet.mnemonic.to_string() } else { hex::encode(wallet.master_public_key.to_bytes()) },
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

pub fn export_chia(wallet: &Wallet, export_hot: bool, label: &str) {
    let (crtbuf, keybuf) = ssl::load_ssl_certs();
    let tlsconn = ssl::build_tls_connector(&crtbuf, &keybuf);
    let (mut ws_stream, _status) = websocket::connect_daemon(&tlsconn);
    
    let cmd = build_add_key_command(wallet, export_hot, label);
    let reqdata = build_websocket_request(cmd);
    
    if let Err(e) = websocket::send_add_key(&mut ws_stream, &reqdata) {
        eprintln!("Export to chia was not successful. Error: {}", e);
    }
}
