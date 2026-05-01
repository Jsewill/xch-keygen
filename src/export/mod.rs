use crate::derive::Wallet;

pub fn run_exports(wallet: &Wallet, exports: &[String], export_hot: bool, label: &str) {
    for export in exports {
        match export.as_str() {
            "sage" => {
                sage::export_sage(wallet, export_hot, label);
            },
            "chia" => {
                chia::export_chia(wallet, export_hot, label);
            },
            _ => {
                if !export.is_empty() {
                    panic!("{} RPC is not yet supported.", export);
                }
            },
        }
    }
}

mod sage;
mod chia;
