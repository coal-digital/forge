use forge_api::instruction::MintV1Args;
use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError
};
use mpl_core::{
  Collection,
  instructions::CreateV2CpiBuilder, 
  types::{Plugin, PluginAuthority, PluginAuthorityPair},
};

pub fn process_mint<'a, 'info>(
  accounts: &'a [AccountInfo<'info>],
  args: MintV1Args,
) -> ProgramResult {
	let [signer, mint_info, collection_info, collection_authority, update_authority, mpl_core_program, system_program] = accounts
	else {
		return Err(ProgramError::NotEnoughAccountKeys);
	};

	let collection = {
		let collection_data = collection_info.data.borrow();
		Collection::from_bytes(&collection_data).unwrap()
	};
	let attributes_plugin = collection.plugin_list.attributes.unwrap();
	let royalties_plugin = collection.plugin_list.royalties.unwrap();
	
	let collection_authority_seeds = &[b"collection_authority".as_ref(), &[args.collection_authority_bump]];

  	CreateV2CpiBuilder::new(mpl_core_program)
        .asset(mint_info)
        .collection(Some(&collection_info))
        .payer(signer)
		.owner(Some(signer))
        .name(collection.base.name.clone())
        .uri(collection.base.uri.clone())
		.authority(Some(collection_authority))
        .plugins(vec![
          PluginAuthorityPair {
            plugin: Plugin::Attributes(attributes_plugin.attributes),
            authority: Some(PluginAuthority::Address {
              address: *update_authority.key,
            }),
          },
          PluginAuthorityPair {
            plugin: Plugin::Royalties(royalties_plugin.royalties),
            authority: Some(PluginAuthority::Address {
              address: *update_authority.key,
            }),
          },
        ])
		.system_program(system_program)
        .invoke_signed(&[collection_authority_seeds])?;

  	Ok(())
}

// let (_, attributes, _) =
// 	fetch_plugin::<BaseAssetV1, Attributes>(&mint_info, PluginType::Attributes).unwrap();

// let durability_value = attributes
// 	.attribute_list
// 	.iter()
// 	.find(|attr| attr.key == "durability")
// 	.map(|attr| &attr.value);

// if let Some(durability) = durability_value {
// 	println!("Durability: {}", durability);
// } else {
// 	println!("Durability attribute not found");
// }
