pub mod consts;
pub mod error;
pub mod event;
pub mod instruction;
pub mod state;

pub(crate) use forge_utils as utils;

use solana_program::declare_id;

declare_id!("HXwf9HxBCZLLJS7uy4q5qAzLiAGLDTp5iWicPNiWC5Vo");
