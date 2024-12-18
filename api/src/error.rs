use num_enum::IntoPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum ForgeError {
    #[error("Auth failed")]
    AuthFailed = 7,
    #[error("Invalid resource")]
    InvalidResource = 8,
}

impl From<ForgeError> for ProgramError {
    fn from(e: ForgeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
