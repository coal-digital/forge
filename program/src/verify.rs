use forge_api::{
    consts::{COLLECTION_AUTHORITY_SEED, INITIALIZER_ADDRESS},
    instruction::VerifyArgs,
    loaders::{load_collection_authority, load_signer}
};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    native_token::LAMPORTS_PER_SOL,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    rent::Rent,
    sysvar::Sysvar,
    system_instruction::transfer 
};

/// Verify the collection authority.
pub fn process_verify<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    args: VerifyArgs,
) -> ProgramResult {
    // Load accounts.
    let [signer, collection_authority, destination_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    
    load_signer(signer)?;
    load_collection_authority(
		collection_authority,
		&[COLLECTION_AUTHORITY_SEED],
		args.collection_authority_bump,
		&forge_api::id(),
	)?;

    // Check signer.
    if signer.key.ne(&INITIALIZER_ADDRESS) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Proxy transfer 0.00001 SOL via the collection authority.
    let rent = Rent::get()?;
    let min_rent = rent.minimum_balance(0);
    let amount = LAMPORTS_PER_SOL / 100_000;
    let transfer_ix = transfer(
        signer.key,
        collection_authority.key,
        amount + min_rent,
    );
    invoke(
        &transfer_ix,
        &[
            signer.clone(),
            collection_authority.clone(),
            system_program.clone()
        ],
    )?;

    let collection_authority_seeds = &[COLLECTION_AUTHORITY_SEED, &[args.collection_authority_bump]];
    let transfer_ix = transfer(
        collection_authority.key,
        destination_info.key,
        amount,
    );
    invoke_signed(
        &transfer_ix,
        &[
            collection_authority.clone(),
            destination_info.clone(),
            system_program.clone()
        ],
        &[collection_authority_seeds],
    )?;

    Ok(())
}
