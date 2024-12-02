use const_crypto::ed25519;
use solana_program::{pubkey, pubkey::Pubkey};

/// The decimal precision of the COAL token.
/// There are 100 billion indivisible units per COAL (called "grains").
pub const TOKEN_DECIMALS: u8 = 11;

/// One COAL, INGOT, ORE and WOOD all have 11 decimals
pub const ONE_TOKEN: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The authority allowed to initialize the program.
pub const INITIALIZER_ADDRESS: Pubkey = pubkey!("FJka1yJHn1SWux2X1o8VqHC8uaAWGv6CbNQvPWLJQufq");

/// Program id for const pda derivations
const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

pub const COAL_PROGRAM_ID: Pubkey = pubkey!("EG67mGGTxMGuPxDLWeccczVecycmpj2SokzpWeBoGVTf");
pub const COAL_PROGRAM_ID_BYTES: [u8; 32] = unsafe { *(&COAL_PROGRAM_ID as *const Pubkey as *const [u8; 32]) };

pub const COAL_UPDATE_AUTHORITY_SEED: &[u8] = b"update_authority";
pub const COAL_UPDATE_AUTHORITY: Pubkey = Pubkey::new_from_array(ed25519::derive_program_address(&[COAL_UPDATE_AUTHORITY_SEED], &COAL_PROGRAM_ID_BYTES).0);

pub const ROYALTIES_BASIS_POINTS: u16 = 400;
pub const ROYALTY_CREATOR_ADDRESS: Pubkey = pubkey!("B7yXtWpKXfwLDGyHLvab7ypZemajAbR2Kvbn2ogNs8J9");

pub const COLLECTION: Pubkey = pubkey!("CuaLHUJA1dyQ6AYcTcMZrCoBqssSJbqkY7VfEEFdxzCk");
pub const COLLECTION_AUTHORITY_SEED: &[u8] = b"collection_authority";
pub const COLLECTION_AUTHORITY_ADDRESS: Pubkey = Pubkey::new_from_array(ed25519::derive_program_address(&[COLLECTION_AUTHORITY_SEED], &PROGRAM_ID).0);

pub const CONFIG_SEED: &[u8] = b"config";
pub const ENHANCER_SEED: &[u8] = b"enhancer";

/// Mints
pub const COAL_MINT_ADDRESS: Pubkey = pubkey!("E3yUqBNTZxV8ELvW99oRLC7z4ddbJqqR4NphwrMug9zu");
pub const INGOT_MINT_ADDEESS: Pubkey = pubkey!("7W6R9rG1kfadLBUWw4mAj9eRCmARtzkbttKVdawVx15V");
pub const WOOD_MINT_ADDRESS: Pubkey = pubkey!("Hrd2en37VJaDspWFq6miK8w6xTuQRTHNaD2e3tqH2uxr");
pub const CHROMIUM_MINT_ADDRESS: Pubkey = pubkey!("Fjgy35F41gm1GNAFKZ5djKo7LMeUoSzFcMK2xC3BfbG8");

pub const TREASURY: &[u8] = b"treasury";

/// The address of the treasury account.
pub const TREASURY_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).0);

/// The bump of the treasury account, for cpis.
pub const TREASURY_BUMP: u8 = ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).1;

pub const ENHANCER_TARGET_SLOT: u64 = 20;
pub const ENHANCE_SLOT_BUFFER: u64 = 6;
pub const ENHANCE_MIN_MULTIPLIER: u64 = 320;
pub const ENHANCE_MAX_MULTIPLIER: u64 = 600;