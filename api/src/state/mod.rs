mod config;
mod enhancer;
mod treasury;

pub use config::*;
pub use enhancer::*;
pub use treasury::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountDiscriminator {
    Config = 100,
    Treasury = 101,
    Enhancer = 102,
}
