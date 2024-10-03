use std::io::Error;
use std::mem::size_of;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction}, pubkey::Pubkey, system_program,
};
use mpl_core::programs::MPL_CORE_ID;

use crate::consts::*;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MintV1Args {
    pub config_bump: u8,
    pub collection_authority_bump: u8,

}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct NewV1Args {
    pub name: String,
    pub uri: String,
    pub multiplier: u64,
    pub durability: u64,
    pub config_bump: u8,
    pub collection_authority_bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct InitializeArgs {
    pub treasury_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
#[rustfmt::skip]
pub enum ForgeInstruction {
    // User
    MintV1(MintV1Args),
    // Admin
    NewV1(NewV1Args),
    Initialize(InitializeArgs),
}

impl ForgeInstruction {
    pub fn try_to_vec(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::with_capacity(size_of::<ForgeInstruction>());
        self.serialize(&mut buf)?;
        Ok(buf)
    }
}

pub fn initialize(signer: Pubkey) -> Instruction {
    let initialize_args = ForgeInstruction::Initialize(InitializeArgs {
        treasury_bump: TREASURY_BUMP,
    });
    
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(TREASURY_ADDRESS, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: [initialize_args.try_to_vec().unwrap()].concat(),
    }
}

    

/// Builds a new instruction.
pub fn new(signer: Pubkey, collection: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());
    let (config, config_bump) = Pubkey::find_program_address(&[CONFIG_SEED, collection.as_ref()], &crate::id());

    let treasury_ingots = spl_associated_token_account::get_associated_token_address(
        &TREASURY_ADDRESS,
        &COAL_MINT_ADDRESS// &INGOT_MINT_ADDEESS,
    );
    let treasury_wood = spl_associated_token_account::get_associated_token_address(
        &TREASURY_ADDRESS,
        &WOOD_MINT_ADDRESS,
    );

    let new_v1_args = ForgeInstruction::NewV1(NewV1Args {
        name: "Miner's Pickaxe".to_string(),
        uri: "https://minechain.gg/metadata.pickaxe.json".to_string(),
        multiplier: 70, // 70% bonus
        durability: 1000, // 1000 uses
        config_bump,
        collection_authority_bump,
    });

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(collection, true),
            AccountMeta::new_readonly(collection_authority, false),
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(TREASURY_ADDRESS, false),
            AccountMeta::new_readonly(MPL_CORE_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new_readonly(COAL_MINT_ADDRESS, false),
            // AccountMeta::new_readonly(INGOT_MINT_ADDEESS, false),
            AccountMeta::new(treasury_ingots, false),
            AccountMeta::new_readonly(WOOD_MINT_ADDRESS, false),
            AccountMeta::new(treasury_wood, false),
        ],
        data: [new_v1_args.try_to_vec().unwrap()].concat(),
    }
}

// signer, mint_info, collection_info, collection_authority, mpl_core_program, system_program
pub fn mint(signer: Pubkey, collection: Pubkey, mint: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());
    let (config, config_bump) = Pubkey::find_program_address(&[CONFIG_SEED, collection.as_ref()], &crate::id());

    let ingot_tokens = spl_associated_token_account::get_associated_token_address(
        &signer,
        &COAL_MINT_ADDRESS // &INGOT_MINT_ADDEESS,
    );
    let treasury_ingots = spl_associated_token_account::get_associated_token_address(
        &TREASURY_ADDRESS,
        &COAL_MINT_ADDRESS // &INGOT_MINT_ADDEESS,
    );
    let wood_tokens = spl_associated_token_account::get_associated_token_address(
        &signer,
        &WOOD_MINT_ADDRESS,
    );
    let treasury_wood = spl_associated_token_account::get_associated_token_address(
        &TREASURY_ADDRESS,
        &WOOD_MINT_ADDRESS,
    );

    let mint_v1_args = ForgeInstruction::MintV1(MintV1Args {
        config_bump,
        collection_authority_bump,
    });

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(collection_authority, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(MPL_CORE_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(ingot_tokens, false),
            AccountMeta::new(treasury_ingots, false),
            AccountMeta::new(wood_tokens, false),
            AccountMeta::new(treasury_wood, false),
        ],
        data: [mint_v1_args.try_to_vec().unwrap()].concat(),
    }
}
