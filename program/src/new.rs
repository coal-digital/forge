use forge_api::{consts::{ROYALTIES_BASIS_POINTS, ROYALTY_CREATOR_ADDRESS}, instruction::NewV1Args};
use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  program_error::ProgramError,
};
use mpl_core::{
  instructions::CreateCollectionV2CpiBuilder, 
  types::{Attribute, Attributes, Creator, Plugin, PluginAuthority, PluginAuthorityPair, Royalties, RuleSet}
};

pub fn process_new<'a, 'info>(
  accounts: &'a [AccountInfo<'info>],
  args: NewV1Args,
) -> ProgramResult {
	let [signer, collection_authority, collection_info, mpl_core_program, system_program] = accounts
	else {
		return Err(ProgramError::NotEnoughAccountKeys);
	};

	let collection_authority_seeds = &[b"collection_authority".as_ref(), &[args.collection_authority_bump]];
	
	CreateCollectionV2CpiBuilder::new(mpl_core_program)
		.collection(collection_info)
		.payer(signer)
		.update_authority(Some(collection_authority))
		.name(args.name)
		.uri(args.uri)
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
					creators: vec![
						Creator {
							address: ROYALTY_CREATOR_ADDRESS,
							percentage: 100,
						},
					],
					rule_set: RuleSet::None,
				}),
				authority: Some(PluginAuthority::UpdateAuthority),
			},
			// PluginAuthorityPair {
			// 	plugin: Plugin::VerifiedCreators(VerifiedCreators {
			// 		signatures: vec![VerifiedCreatorsSignature {
			// 				address: *collection_authority.key,
			// 				verified: true,
			// 		}],
			// 	}),
			// 	authority: Some(PluginAuthority::UpdateAuthority),
			// },
		])
		.system_program(system_program)
		.invoke_signed(&[collection_authority_seeds])?;

  	Ok(())
}
