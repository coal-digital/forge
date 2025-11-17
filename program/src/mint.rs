use forge_api::prelude::*;
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    types::{Attribute, Attributes, Plugin, PluginAuthority, PluginAuthorityPair},
    Collection,
};
use solana_program::msg;
use steel::*;

pub fn process_mint<'a, 'info>(accounts: &'a [AccountInfo<'info>], data: &[u8]) -> ProgramResult {
    let args = MintV1::try_from_bytes(data).unwrap();
    let (required_accounts, remaining_accounts) = accounts.split_at(8);
    let [signer, mint_info, collection_info, collection_authority, config_info, mpl_core_program, token_program, system_program] =
        required_accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer.is_signer()?;
    mpl_core_program.is_program(&mpl_core::ID)?;
    token_program.is_program(&TOKEN_PROGRAM_ID)?;
    system_program.is_program(&solana_program::system_program::ID)?;

    config_info.has_seeds(&[CONFIG_SEED, collection_info.key.as_ref()], &forge_api::ID)?;
    let config = config_info.as_account::<Config>(&forge_api::ID)?;

    for i in 0..config.ingredients.len() {
        let ingredient = config.ingredients[i];
        let amount = config.amounts[i];
        msg!("Ingredient: {:?}, amount: {:?}", ingredient, amount);
        if amount == 0 {
            continue;
        }

        let mint_info = &remaining_accounts[i * 2];
        let ingredient_tokens_info = &remaining_accounts[i * 2 + 1];

        if ingredient.ne(&mint_info.key) {
            return Err(ProgramError::InvalidAccountData);
        }

        ingredient_tokens_info.as_associated_token_account(signer.key, mint_info.key)?;

        // Burn ingredient tokens
        burn(
            ingredient_tokens_info,
            mint_info,
            signer,
            token_program,
            amount,
        )?;
    }

    let collection: Box<Collection> = {
        let collection_data = collection_info.data.borrow();
        Collection::from_bytes(&collection_data).unwrap()
    };
    let royalties_plugin = collection.plugin_list.royalties.unwrap();

    let mut attribute_list = vec![
        Attribute {
            key: "multiplier".to_string(),
            value: "300".to_string(),
        },
        Attribute {
            key: "rarity".to_string(),
            value: "common".to_string(),
        },
    ];

    attribute_list.push(Attribute {
        key: "resource".to_string(),
        value: "coal".to_string(),
    });
    attribute_list.push(Attribute {
        key: "durability".to_string(),
        value: "1000".to_string(),
    });

    let name = "Miner's Pickaxe".to_string();
    let uri = "https://minechain.gg/metadata.pickaxe.json".to_string();

    let collection_authority_seeds =
        &[COLLECTION_AUTHORITY_SEED, &[args.collection_authority_bump]];

    CreateV2CpiBuilder::new(mpl_core_program)
        .asset(mint_info)
        .collection(Some(&collection_info))
        .payer(signer)
        .owner(Some(signer))
        .name(name)
        .uri(uri)
        .authority(Some(collection_authority))
        .plugins(vec![
            PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes { attribute_list }),
                authority: Some(PluginAuthority::Address {
                    address: COAL_UPDATE_AUTHORITY,
                }),
            },
            PluginAuthorityPair {
                plugin: Plugin::Royalties(royalties_plugin.royalties),
                authority: Some(PluginAuthority::Address {
                    address: COAL_UPDATE_AUTHORITY,
                }),
            },
        ])
        .system_program(system_program)
        .invoke_signed(&[collection_authority_seeds])?;

    Ok(())
}
