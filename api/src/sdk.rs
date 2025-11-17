use mpl_core;
use steel::*;

use crate::prelude::*;

/// Builds a new instruction.
pub fn new(signer: Pubkey, collection: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) =
        Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());
    let (config, config_bump) =
        Pubkey::find_program_address(&[CONFIG_SEED, collection.as_ref()], &crate::id());

    let mut name = [0u8; 64];
    let name_bytes = b"Miner's Pickaxe";
    name[..name_bytes.len()].copy_from_slice(name_bytes);

    let mut uri = [0u8; 128];
    let uri_bytes = b"https://minechain.gg/metadata.pickaxe.json";
    uri[..uri_bytes.len()].copy_from_slice(uri_bytes);

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(collection, true),
            AccountMeta::new_readonly(collection_authority, false),
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(mpl_core::ID, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new_readonly(COAL_MINT_ADDRESS, false),
            AccountMeta::new_readonly(WOOD_MINT_ADDRESS, false),
        ],
        data: NewV1 {
            name,
            uri,
            multiplier: 70,
            durability: 1000,
            amounts: [ONE_TOKEN.saturating_mul(1), 0, 0],
            ingredients: [
                COAL_MINT_ADDRESS,
                solana_program::system_program::ID,
                solana_program::system_program::ID,
            ],
            config_bump,
            collection_authority_bump,
            _padding: [0; 6],
        }
        .to_bytes(),
    }
}

// signer, mint_info, collection_info, collection_authority, mpl_core_program, system_program
pub fn mint(signer: Pubkey, collection: Pubkey, mint: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) =
        Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());
    let (config, config_bump) =
        Pubkey::find_program_address(&[CONFIG_SEED, collection.as_ref()], &crate::id());

    let ingot_tokens =
        spl_associated_token_account::get_associated_token_address(&signer, &INGOT_MINT_ADDEESS);
    let wood_tokens =
        spl_associated_token_account::get_associated_token_address(&signer, &WOOD_MINT_ADDRESS);

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(collection_authority, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(mpl_core::ID, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(INGOT_MINT_ADDEESS, false),
            AccountMeta::new(ingot_tokens, false),
            AccountMeta::new(WOOD_MINT_ADDRESS, false),
            AccountMeta::new(wood_tokens, false),
        ],
        data: MintV1 {
            config_bump,
            collection_authority_bump,
        }
        .to_bytes(),
    }
}
