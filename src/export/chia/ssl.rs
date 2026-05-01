use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use native_tls::Identity;
use native_tls::TlsConnector;

pub fn load_ssl_certs() -> (Vec<u8>, Vec<u8>) {
    let homepbuf: PathBuf = dirs::home_dir().expect("Couldn't get path to the user's home directory.");
    let nodepbuf: PathBuf = homepbuf.join(".chia/mainnet/config/ssl/daemon");
    let crtpbuf: PathBuf = nodepbuf.join("private_daemon.crt");
    let keypbuf: PathBuf = nodepbuf.join("private_daemon.key");
    
    let mut crtbuf = Vec::new();
    let mut keybuf = Vec::new();
    
    File::open(&crtpbuf).expect(&format!("Couldn't open chia SSL certificate file: {}", crtpbuf.display()))
        .read_to_end(&mut crtbuf).expect(&format!("Couldn't read from chia SSL certificate file: {}", crtpbuf.display()));
    File::open(&keypbuf).expect(&format!("Couldn't open chia SSL key file: {}", keypbuf.display()))
        .read_to_end(&mut keybuf).expect(&format!("Couldn't read from chia SSL key file: {}", keypbuf.display()));
    
    (crtbuf, keybuf)
}

pub fn build_tls_connector(crtbuf: &[u8], keybuf: &[u8]) -> TlsConnector {
    let ident = Identity::from_pkcs8(crtbuf, keybuf).expect("Couldn't produce an identity from the chia SSL certificate pair.");
    TlsConnector::builder()
        .identity(ident)
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to build chia rpc websocket request")
}
