use steel::*;

use super::ForgeAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Config {
    pub amounts: [u64; 3],
    pub ingredients: [Pubkey; 3],
}

account!(ForgeAccount, Config);
