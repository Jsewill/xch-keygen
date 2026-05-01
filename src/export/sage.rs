use sage_api::{ImportKey as SageImportKey, ImportKeyResponse as SageImportKeyResponse};
use sage_client::{self, SageRpcError};

use crate::derive::Wallet;

pub fn export_sage(wallet: &Wallet, export_hot: bool, label: &str) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    let sage_rpc_client = sage_client::Client::new().expect("Failed to set up Sage RPC client.");
    let ikreq: SageImportKey = SageImportKey {
        name: label.to_string(),
        key: if export_hot { wallet.mnemonic.to_string() } else { hex::encode(wallet.mpk.to_bytes()) },
        derivation_index: *wallet.indices.last().unwrap_or(&wallet.addresses),
        emoji: Some("".to_string()),
        save_secrets: true,
        login: false,
    };
    
    rt.block_on(async {
        let _resp: Result<SageImportKeyResponse, SageRpcError> = sage_rpc_client.import_key(ikreq).await;
    });
}
