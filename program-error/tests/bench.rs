//! Bench case with manual implementations
use spl_program_error::*;

/// Example error
#[derive(Clone, Debug, Eq, thiserror::Error, num_derive::FromPrimitive, PartialEq)]
pub enum ExampleError {
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
}

impl From<ExampleError> for solana_program_error::ProgramError {
    fn from(e: ExampleError) -> Self {
        solana_program_error::ProgramError::Custom(e as u32)
    }
}

impl solana_program_error::PrintProgramError for ExampleError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + solana_program_error::PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            ExampleError::MintHasNoMintAuthority => {
                solana_msg::msg!("Mint has no mint authority")
            }
            ExampleError::IncorrectMintAuthority => {
                solana_msg::msg!("Incorrect mint authority has signed the instruction")
            }
        }
    }
}

/// Tests that all macros compile
#[test]
fn test_macros_compile() {
    let _ = ExampleError::MintHasNoMintAuthority;
}
