use std::net::TcpStream;
use std::str::FromStr;
use std::time::Duration;

use native_tls::TlsConnector;
use serde::Serialize;
use tungstenite::{http::Uri, stream::MaybeTlsStream};

use crate::chia_rpc::daemon::ResponseData;

pub fn connect_daemon(tlsconn: &TlsConnector) -> (tungstenite::WebSocket<MaybeTlsStream<TcpStream>>, String) {
    let uri = Uri::from_str("wss://localhost:55400/").unwrap();
    let host = uri.host().unwrap().to_string();
    let port = uri.port().unwrap().to_string();
    
    let tcps = TcpStream::connect(format!("{}:{}", host, port)).expect("Couldn't establish a TCP connection with the chia daemon. Make sure the chia daemon is started.");
    tcps.set_read_timeout(Some(Duration::from_secs(10))).expect("Couldn't set read timeout on TCP connection.");
    tcps.set_write_timeout(Some(Duration::from_secs(10))).expect("Couldn't set write timeout on TCP connection.");
    
    let tlss = tlsconn.connect(&host, tcps).expect("Couldn't establish TLS connection with the chia daemon. Make sure the daemon ssl certificates are present in the chia data directory.");
    let tlss = MaybeTlsStream::NativeTls(tlss);
    
    let req = tungstenite::handshake::client::Request::builder()
        .uri(uri.to_string())
        .header("Host", uri.host().unwrap())
        .header("Upgrade", "websocket")
        .header("Connection", "Upgrade")
        .header("Sec-WebSocket-Key", tungstenite::handshake::client::generate_key())
        .header("Sec-WebSocket-Version", "13")
        .body(()).expect("Couldn't build websocket handshake request for chia daemon.");
    
    let (ws_stream, resp) = tungstenite::client(req, tlss).expect("Couldn't complete websocket handshake with chia daemon.");
    
    (ws_stream, resp.status().to_string())
}

pub fn send_add_key<T: Serialize>(ws_stream: &mut tungstenite::WebSocket<MaybeTlsStream<TcpStream>>, req: &T) -> Result<bool, String> {
    let reqjson = serde_json::to_string(req).expect("Couldn't serialize chia daemon websocket request.");
    ws_stream.send(tungstenite::Message::Text(reqjson.into())).expect("Couldn't make the websocket request to the chia daemon.");
    
    if let Ok(msg) = ws_stream.read() {
        let respjson: ResponseData = serde_json::from_str(msg.to_text().unwrap()).unwrap();
        if !respjson.success {
            return Err(respjson.error.unwrap_or("No error was provided.".to_string()));
        }
        Ok(true)
    } else {
        Err("No response from chia daemon.".to_string())
    }
}
