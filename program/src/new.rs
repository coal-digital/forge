use forge_api::prelude::*;
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder,
    types::{
        Attribute, Attributes, Creator, Plugin, PluginAuthority, PluginAuthorityPair, Royalties,
        RuleSet,
    },
};
use steel::*;

pub fn process_new<'a, 'info>(accounts: &'a [AccountInfo<'info>], data: &[u8]) -> ProgramResult {
    let args = NewV1::try_from_bytes(data)?;
    let (required_accounts, additional_accounts) = accounts.split_at(8);
    let [signer, collection_info, collection_authority, config_info, mpl_core_program, token_program, system_program] =
        required_accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts using steel SDK
    signer.is_signer()?;
    collection_authority.has_seeds(
        &[COLLECTION_AUTHORITY_SEED, &[args.collection_authority_bump]],
        &forge_api::id(),
    )?;
    mpl_core_program.is_program(&mpl_core::ID)?;
    token_program.is_program(&TOKEN_PROGRAM_ID)?;
    system_program.is_program(&system_program::ID)?;

    // Check signer.
    if signer.key.ne(&INITIALIZER_ADDRESS) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Initialize config PDA
    create_program_account::<Config>(
        config_info,
        system_program,
        signer,
        &forge_api::id(),
        &[CONFIG_SEED, collection_info.key.as_ref()],
    )?;

    // Initialize config data
    let config = config_info.as_account_mut::<Config>(&forge_api::id())?;
    config.amounts = args.amounts;
    config.ingredients = args.ingredients;

    // Validate mints
    for i in 0..config.ingredients.len() {
        let ingredient = config.ingredients[i];

        if ingredient.eq(&solana_program::system_program::ID) {
            continue;
        }

        let mint_info = &additional_accounts[i];
        if mint_info.key.ne(&ingredient) {
            return Err(ProgramError::InvalidAccountData);
        }
        mint_info.as_mint()?;
        // TODO burn ingredient tokens
    }

    let collection_authority_seeds =
        &[COLLECTION_AUTHORITY_SEED, &[args.collection_authority_bump]];

    // Convert byte arrays to strings
    let name = String::from_utf8_lossy(&args.name)
        .trim_end_matches('\0')
        .to_string();
    let uri = String::from_utf8_lossy(&args.uri)
        .trim_end_matches('\0')
        .to_string();

    CreateCollectionV2CpiBuilder::new(mpl_core_program)
        .collection(collection_info)
        .payer(signer)
        .update_authority(Some(collection_authority))
        .name(name)
        .uri(uri)
        .plugins(vec![
            PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute {
                            key: "multiplier".to_string(),
                            value: args.multiplier.to_string(),
                        },
                        Attribute {
                            key: "durability".to_string(),
                            value: args.durability.to_string(),
                        },
                        Attribute {
                            key: "rarity".to_string(),
                            value: "common".to_string(),
                        },
                        Attribute {
                            key: "resource".to_string(),
                            value: "coal".to_string(),
                        },
                    ],
                }),
                authority: Some(PluginAuthority::UpdateAuthority),
            },
            PluginAuthorityPair {
                plugin: Plugin::Royalties(Royalties {
                    basis_points: ROYALTIES_BASIS_POINTS,
                    creators: vec![Creator {
                        address: ROYALTY_CREATOR_ADDRESS,
                        percentage: 100,
                    }],
                    rule_set: RuleSet::None,
                }),
                authority: Some(PluginAuthority::UpdateAuthority),
            },
        ])
        .system_program(system_program)
        .invoke_signed(&[collection_authority_seeds])?;

    Ok(())
}
