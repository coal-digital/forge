mod mint;
mod new;

use mint::*;
use new::*;

use forge_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&forge_api::ID, program_id, data)?;

    match ix {
        ForgeInstruction::NewV1 => process_new(accounts, data)?,
        ForgeInstruction::MintV1 => process_mint(accounts, data)?,
    }

    Ok(())
}
