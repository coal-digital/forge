mod config;
mod treasury;

pub use config::*;
pub use treasury::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum ForgeAccount {
    Config = 100,
    Treasury = 101,
    Enhancer = 102,
}
