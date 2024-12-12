use std::io::Error;
use std::mem::size_of;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    sysvar
};
use mpl_core::programs::MPL_CORE_ID;

use crate::consts::*;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MintV1Args {
    pub resource: String,
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
    pub ingredients: [Pubkey; 3],
    pub amounts: [u64; 3],
    pub config_bump: u8,
    pub collection_authority_bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct InitializeArgs {
    pub treasury_bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct VerifyArgs {
    pub collection_authority_bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct InitializeEnhanceArgs {
    pub enhancer_bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct EnhanceArgs {
    pub enhancer_bump: u8,
    pub collection_authority_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
#[rustfmt::skip]
pub enum ForgeInstruction {
    // User
    MintV1(MintV1Args),
    // Admin
    NewV1(NewV1Args),
    Initialize(InitializeArgs),
    Verify(VerifyArgs),
    InitializeEnhance(InitializeEnhanceArgs),
    Enhance(EnhanceArgs),
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

pub fn verify(signer: Pubkey, destination: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());

    let verify_args: ForgeInstruction = ForgeInstruction::Verify(VerifyArgs {
        collection_authority_bump,
    });
    
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(collection_authority, false),
            AccountMeta::new(destination, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: [verify_args.try_to_vec().unwrap()].concat(),
    }
}   

/// Builds a new instruction.
pub fn new(signer: Pubkey, collection: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());
    let (config, config_bump) = Pubkey::find_program_address(&[CONFIG_SEED, collection.as_ref()], &crate::id());

    let new_v1_args = ForgeInstruction::NewV1(NewV1Args {
        name: "Miner's Pickaxe".to_string(),
        uri: "https://minechain.gg/metadata.pickaxe.json".to_string(),
        multiplier: 70, // 70% bonus
        durability: 1000, // 1000 uses
        amounts: [ONE_TOKEN.saturating_mul(1), 0, 0],
        ingredients: [COAL_MINT_ADDRESS, solana_program::system_program::ID, solana_program::system_program::ID],
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
            AccountMeta::new_readonly(MPL_CORE_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new_readonly(COAL_MINT_ADDRESS, false),
            AccountMeta::new_readonly(WOOD_MINT_ADDRESS, false),
        ],
        data: [new_v1_args.try_to_vec().unwrap()].concat(),
    }
}

// signer, mint_info, collection_info, collection_authority, mpl_core_program, system_program
pub fn mint(signer: Pubkey, collection: Pubkey, mint: Pubkey, resource: String) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());
    let (config, config_bump) = Pubkey::find_program_address(&[CONFIG_SEED, collection.as_ref()], &crate::id());

    let ingot_tokens = spl_associated_token_account::get_associated_token_address(
        &signer,
        &INGOT_MINT_ADDEESS,
    );
    let wood_tokens = spl_associated_token_account::get_associated_token_address(
        &signer,
        &WOOD_MINT_ADDRESS,
    );

    let mint_v1_args: ForgeInstruction = ForgeInstruction::MintV1(MintV1Args {
        resource,
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
            AccountMeta::new(INGOT_MINT_ADDEESS, false),
            AccountMeta::new(ingot_tokens, false),
            AccountMeta::new(WOOD_MINT_ADDRESS, false),
            AccountMeta::new(wood_tokens, false),
        ],
        data: [mint_v1_args.try_to_vec().unwrap()].concat(),
    }
}

// signer, asset_info, enhancer_info, chromium_mint_info, chromium_tokens_info, slot_hashes_sysvar, system_program
pub fn init_enhance(signer: Pubkey, asset: Pubkey) -> Instruction {
    let (enhancer, enhancer_bump) = Pubkey::find_program_address(&[ENHANCER_SEED, signer.as_ref(), asset.as_ref()], &crate::id());

    let chromium_tokens = spl_associated_token_account::get_associated_token_address(
        &signer,
        &CHROMIUM_MINT_ADDRESS,
    );

    let init_enhance_args: ForgeInstruction = ForgeInstruction::InitializeEnhance(InitializeEnhanceArgs {
        enhancer_bump,
    });

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(asset, false),
            AccountMeta::new(enhancer, false),
            AccountMeta::new(CHROMIUM_MINT_ADDRESS, false),
            AccountMeta::new(chromium_tokens, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: [init_enhance_args.try_to_vec().unwrap()].concat(),
    }
}

// signer, asset_info, new_mint_info, collection_info, collection_authority, enhancer_info, mpl_core_program, system_program, slot_hashes_sysvar
pub fn enhance(signer: Pubkey, asset: Pubkey, new_mint: Pubkey, collection: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());
    let (enhancer, enhancer_bump) = Pubkey::find_program_address(&[ENHANCER_SEED, signer.as_ref(), asset.as_ref()], &crate::id());

    let enhance_args: ForgeInstruction = ForgeInstruction::Enhance(EnhanceArgs {
        enhancer_bump,
        collection_authority_bump,
    });

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(new_mint, true),
            AccountMeta::new(asset, false),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(collection_authority, false),
            AccountMeta::new(enhancer, false),
            AccountMeta::new_readonly(MPL_CORE_ID, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
        ],
        data: [enhance_args.try_to_vec().unwrap()].concat(),
    }
}