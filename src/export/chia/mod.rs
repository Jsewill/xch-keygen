use crate::derive::Wallet;
use crate::output;

mod ssl;
mod websocket;

pub fn export_chia(wallet: &Wallet, export_hot: bool, label: &str) {
    let (crtbuf, keybuf) = ssl::load_ssl_certs();
    let tlsconn = ssl::build_tls_connector(&crtbuf, &keybuf);
    let (mut ws_stream, _status) = websocket::connect_daemon(&tlsconn);
    
    let cmd = output::build_add_key_command(wallet, export_hot, label);
    let reqdata = output::build_websocket_request(cmd);
    
    if let Err(e) = websocket::send_add_key(&mut ws_stream, &reqdata) {
        eprintln!("Export to chia was not successful. Error: {}", e);
    }
}
