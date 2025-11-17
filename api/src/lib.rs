pub mod consts;
pub mod error;
pub mod instruction;
pub mod sdk;
pub mod state;
pub mod utils;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
    pub use crate::utils::*;
}

use steel::*;

declare_id!("HXwf9HxBCZLLJS7uy4q5qAzLiAGLDTp5iWicPNiWC5Vo");
