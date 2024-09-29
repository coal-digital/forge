
use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

use crate::utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    pub amounts: [u64; 3],
    pub ingredients: [Pubkey; 3],
}

impl Discriminator for Config {
    fn discriminator() -> u8 {
        AccountDiscriminator::Config.into()
    }
}

impl_to_bytes!(Config);
impl_account_from_bytes!(Config);
