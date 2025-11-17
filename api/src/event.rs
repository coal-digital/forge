use steel::*;

use crate::utils::impl_to_bytes;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct CraftEvent {
    pub mint: Pubkey,
    pub collection: Pubkey,
}

impl_to_bytes!(CraftEvent);
