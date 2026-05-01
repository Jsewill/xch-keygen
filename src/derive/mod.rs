use bip39::Mnemonic;
use chia::bls::{
    master_to_wallet_hardened_intermediate, master_to_wallet_unhardened_intermediate,
    PublicKey, SecretKey,
};
use rand::RngCore;
use rand_chacha::ChaCha20Rng;

use crate::cli::Args;

pub struct Wallet {
    pub mnemonic: Mnemonic,
    pub msk: SecretKey,
    pub hi: SecretKey,
    pub mpk: PublicKey,
    pub ui: PublicKey,
    pub fp: u32,
    pub fsk: SecretKey,
    pub fpk: PublicKey,
    pub ppk: PublicKey,
    pub indices: Vec<u32>,
    pub addresses: u32,
}

impl Wallet {
    pub fn new(mnemonic: &Mnemonic, args: &Args, rng: &mut ChaCha20Rng) -> Self {
        let msk = SecretKey::from_seed(&mnemonic.to_seed(""));
        let hi = master_to_wallet_hardened_intermediate(&msk);
        let mpk = msk.public_key();
        let ui = master_to_wallet_unhardened_intermediate(&mpk);
        let fp = mpk.get_fingerprint();
        let fsk = msk.derive_hardened(12381).derive_hardened(8444).derive_hardened(0).derive_hardened(0);
        let fpk = fsk.public_key();
        let ppk = msk.derive_hardened(12381).derive_hardened(8444).derive_hardened(1).derive_hardened(0).public_key();

        let addresses = args.addresses.unwrap_or(10);
        let offset = args.offset.unwrap_or(0);
        let skip = args.skip.unwrap_or(0);
        let random = args.random;
        let mut height = args.height.unwrap_or(offset + addresses);
        if height < offset + addresses {
            height = offset + addresses;
        }

        let indices = addresses::generate_indices(offset, addresses, skip, random, height, rng);

        Wallet {
            mnemonic: mnemonic.clone(),
            msk,
            hi,
            mpk,
            ui,
            fp,
            fsk,
            fpk,
            ppk,
            indices,
            addresses,
        }
    }
}

pub fn generate_mnemonic(words: u8, rng: &mut ChaCha20Rng) -> Mnemonic {
    let mut entropy: [u8; 32] = [0; 32];
    rng.fill_bytes(&mut entropy);
    match words {
        12 => Mnemonic::from_entropy(&entropy[..16]).expect("Could not generate mnemonic from entropy"),
        _ => Mnemonic::from_entropy(&entropy).expect("Could not generate mnemonic from entropy"),
    }
}

pub fn derive_indices(args: &Args, rng: &mut ChaCha20Rng) -> Vec<u32> {
    let offset = args.offset.unwrap_or(0);
    let addresses = args.addresses.unwrap_or(10);
    let skip = args.skip.unwrap_or(0);
    let random = args.random;
    let mut height = args.height.unwrap_or(offset + addresses);
    if height < offset + addresses {
        height = offset + addresses;
    }
    addresses::generate_indices(offset, addresses, skip, random, height, rng)
}

pub fn fingerprint_name(fp: u32) -> String {
    naming::fingerprint_name(fp)
}

mod addresses;
mod naming;
