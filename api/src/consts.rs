use const_crypto::ed25519;
use solana_program::{pubkey, pubkey::Pubkey};


/// The authority allowed to initialize the program.
pub const INITIALIZER_ADDRESS: Pubkey = pubkey!("FJka1yJHn1SWux2X1o8VqHC8uaAWGv6CbNQvPWLJQufq");

/// Program id for const pda derivations
const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

pub const COAL_ADDRESS: Pubkey = pubkey!("EG67mGGTxMGuPxDLWeccczVecycmpj2SokzpWeBoGVTf");
pub const UPDATE_AUTHORITY_SEED: &[u8] = b"update_authority";

pub const ROYALTIES_BASIS_POINTS: u16 = 400;
pub const ROYALTY_CREATOR_ADDRESS: Pubkey = pubkey!("B7yXtWpKXfwLDGyHLvab7ypZemajAbR2Kvbn2ogNs8J9");

pub const COLLECTION_AUTHORITY_SEED: &[u8] = b"collection_authority";
pub const COLLECTION_AUTHORITY_ADDRESS: Pubkey = Pubkey::new_from_array(ed25519::derive_program_address(&[COLLECTION_AUTHORITY_SEED], &PROGRAM_ID).0);