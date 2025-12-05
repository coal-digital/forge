mod mint;
mod new;
mod update;

use mint::*;
use new::*;
use update::*;

use forge_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    solana_program::msg!("Raw instruction data length: {}", data.len());
    solana_program::msg!(
        "Raw instruction data (first 32 bytes): {:?}",
        &data[..data.len().min(32)]
    );

    let (ix, data) = parse_instruction(&forge_api::ID, program_id, data)?;

    solana_program::msg!("Parsed instruction: {:?}", ix);
    solana_program::msg!(
        "Data after parse (length {}): {:?}",
        data.len(),
        &data[..data.len().min(32)]
    );

    match ix {
        ForgeInstruction::NewV1 => process_new(accounts, data)?,
        ForgeInstruction::MintV1 => process_mint(accounts, data)?,
        ForgeInstruction::UpdateV1 => process_update(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
