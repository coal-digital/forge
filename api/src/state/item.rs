
use bytemuck::{Pod, Zeroable};

use crate::utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

/// Item is a struct which manages the durability of an item.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Item {
    /// The remaining durability of the item.
    pub durability: u64,
}

impl Discriminator for Item {
    fn discriminator() -> u8 {
        AccountDiscriminator::Item.into()
    }
}

impl_to_bytes!(Item);
impl_account_from_bytes!(Item);
