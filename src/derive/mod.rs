//! BLS key derivation and wallet structure.
//!
//! This module handles Chia BLS key derivation from a mnemonic seed phrase,
//! producing a [`Wallet`] with all derived keys and generated addresses.

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
    pub master_secret_key: SecretKey,
    pub hardened_intermediate: SecretKey,
    pub master_public_key: PublicKey,
    pub unhardened_intermediate: PublicKey,
    pub fingerprint: u32,
    pub farmer_secret_key: SecretKey,
    pub farmer_public_key: PublicKey,
    pub pool_public_key: PublicKey,
    pub indices: Vec<u32>,
    pub addresses: u32,
}

impl Wallet {
    pub fn new(mnemonic: &Mnemonic, args: &Args, rng: &mut ChaCha20Rng) -> Self {
        let master_secret_key = SecretKey::from_seed(&mnemonic.to_seed(""));
        let hardened_intermediate = master_to_wallet_hardened_intermediate(&master_secret_key);
        let master_public_key = master_secret_key.public_key();
        let unhardened_intermediate = master_to_wallet_unhardened_intermediate(&master_public_key);
        let fingerprint = master_public_key.get_fingerprint();
        let farmer_secret_key = master_secret_key.derive_hardened(12381).derive_hardened(8444).derive_hardened(0).derive_hardened(0);
        let farmer_public_key = farmer_secret_key.public_key();
        let pool_public_key = master_secret_key.derive_hardened(12381).derive_hardened(8444).derive_hardened(1).derive_hardened(0).public_key();

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
            master_secret_key,
            hardened_intermediate,
            master_public_key,
            unhardened_intermediate,
            fingerprint,
            farmer_secret_key,
            farmer_public_key,
            pool_public_key,
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

pub use naming::fingerprint_name;

mod addresses;
mod naming;
