

mod new;
mod mint;

use new::*;
use mint::*;

use forge_api::instruction::*;
use borsh::BorshDeserialize;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&forge_api::id()) {
        println!("Program ID mismatch");
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction: ForgeInstruction = ForgeInstruction::try_from_slice(data).or(Err(ProgramError::InvalidInstructionData))?;
    println!("Validated instruction data");
    
    match instruction {
        ForgeInstruction::NewV1(args) => process_new(accounts, args)?,
        ForgeInstruction::MintV1(args) => process_mint(accounts, args)?,
    }

    Ok(())
}
