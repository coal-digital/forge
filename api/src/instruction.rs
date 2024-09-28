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
pub struct NewV1Args {
    pub name: String,
    pub uri: String,
    pub multiplier: u64,
    pub durability: u64,
    pub collection_authority_bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct MintV1Args {
    pub collection_authority_bump: u8,

}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
#[rustfmt::skip]
pub enum ForgeInstruction {
    // User
    MintV1(MintV1Args),
    // Admin
    NewV1(NewV1Args),
}

impl ForgeInstruction {
    pub fn try_to_vec(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::with_capacity(size_of::<ForgeInstruction>());
        self.serialize(&mut buf)?;
        Ok(buf)
    }
}

/// Builds a new instruction.
pub fn new(signer: Pubkey, mint: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());

    let new_v1_args = ForgeInstruction::NewV1(NewV1Args {
        name: "test2".to_string(),
        uri: "test2".to_string(),
        multiplier: 50,
        durability: 2000,
        collection_authority_bump,
    });

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(collection_authority, false),
            AccountMeta::new(mint, true),
            AccountMeta::new_readonly(MPL_CORE_ID, false),
            AccountMeta::new(system_program::id(), false),
        ],
        data: [new_v1_args.try_to_vec().unwrap()].concat(),
    }
}

// signer, mint_info, collection_info, collection_authority, mpl_core_program, system_program
pub fn mint(signer: Pubkey, collection: Pubkey, mint: Pubkey) -> Instruction {
    let (collection_authority, collection_authority_bump) = Pubkey::find_program_address(&[COLLECTION_AUTHORITY_SEED], &crate::id());

    let mint_v1_args = ForgeInstruction::MintV1(MintV1Args {
        collection_authority_bump,
    });

    let (update_authority, _) = Pubkey::find_program_address(&[UPDATE_AUTHORITY_SEED], &COAL_ADDRESS);

    println!("program_id: {}", crate::id());

    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(collection_authority, false),
            AccountMeta::new_readonly(update_authority, false),
            AccountMeta::new_readonly(MPL_CORE_ID, false),
            AccountMeta::new(system_program::id(), false),
        ],
        data: [mint_v1_args.try_to_vec().unwrap()].concat(),
    }
}