use forge_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_update<'a, 'info>(accounts: &'a [AccountInfo<'info>], data: &[u8]) -> ProgramResult {
    msg!("process_update: data length = {}", data.len());
    msg!(
        "process_update: expected size = {}",
        std::mem::size_of::<UpdateV1>()
    );

    if data.len() >= 96 {
        msg!("Ingredients bytes (first 96): {:?}", &data[..96]);
    }
    if data.len() >= 120 {
        msg!("Amounts bytes (bytes 96-120): {:?}", &data[96..120]);
    }
    if data.len() >= 128 {
        msg!("Padding bytes (bytes 120-128): {:?}", &data[120..128]);
    }
    msg!("Full data: {:?}", data);

    let mut aligned_data = [0u8; 128];
    aligned_data.copy_from_slice(data);
    let args = UpdateV1::try_from_bytes(&aligned_data)?;
    msg!("Successfully deserialized UpdateV1!");
    msg!("Ingredients: {:?}", args.ingredients);
    msg!("Amounts: {:?}", args.amounts);
    let [signer, collection_info, config_info, token_program, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts using steel SDK
    signer.is_signer()?;
    token_program.is_program(&TOKEN_PROGRAM_ID)?;
    system_program.is_program(&system_program::ID)?;

    // Check signer.
    if signer.key.ne(&INITIALIZER_ADDRESS) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate config PDA
    config_info.has_seeds(
        &[CONFIG_SEED, collection_info.key.as_ref()],
        &forge_api::id(),
    )?;

    // Update config data
    let config = config_info.as_account_mut::<Config>(&forge_api::id())?;
    config.amounts = args.amounts;
    config.ingredients = args.ingredients;

    Ok(())
}
