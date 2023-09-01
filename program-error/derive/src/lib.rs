//! Crate defining a procedural macro for building Solana program errors

// Required to include `#[allow(clippy::integer_arithmetic)]`
// below since the tokens generated by `quote!` in the implementation
// for `MacroType::PrintProgramError` and `MacroType::SplProgramError`
// trigger the lint upstream through `quote_token_with_context` within the
// `quote` crate
//
// Culprit is `macro_impl.rs:66`
#![allow(clippy::integer_arithmetic)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate proc_macro;

mod macro_impl;
mod parser;

use {
    crate::parser::SplProgramErrorArgs,
    macro_impl::MacroType,
    proc_macro::TokenStream,
    syn::{parse_macro_input, ItemEnum},
};

/// Derive macro to add `Into<solana_program::program_error::ProgramError>`
/// trait
#[proc_macro_derive(IntoProgramError)]
pub fn into_program_error(input: TokenStream) -> TokenStream {
    let ItemEnum { ident, .. } = parse_macro_input!(input as ItemEnum);
    MacroType::IntoProgramError { ident }
        .generate_tokens()
        .into()
}

/// Derive macro to add `solana_program::decode_error::DecodeError` trait
#[proc_macro_derive(DecodeError)]
pub fn decode_error(input: TokenStream) -> TokenStream {
    let ItemEnum { ident, .. } = parse_macro_input!(input as ItemEnum);
    MacroType::DecodeError { ident }.generate_tokens().into()
}

/// Derive macro to add `solana_program::program_error::PrintProgramError` trait
#[proc_macro_derive(PrintProgramError)]
pub fn print_program_error(input: TokenStream) -> TokenStream {
    let ItemEnum {
        ident, variants, ..
    } = parse_macro_input!(input as ItemEnum);
    MacroType::PrintProgramError { ident, variants }
        .generate_tokens()
        .into()
}

/// Proc macro attribute to turn your enum into a Solana Program Error
///
/// Adds:
/// - `Clone`
/// - `Debug`
/// - `Eq`
/// - `PartialEq`
/// - `thiserror::Error`
/// - `num_derive::FromPrimitive`
/// - `Into<solana_program::program_error::ProgramError>`
/// - `solana_program::decode_error::DecodeError`
/// - `solana_program::program_error::PrintProgramError`
///
/// Optionally, you can add `hash_error_code_start: u32` argument to create
/// a unique `u32` _starting_ error codes from the names of the enum variants.
/// Notes:
/// - The _error_ variant will start at this value, and the rest will be
/// incremented by one
/// - The value provided is only for code readability, the actual error code
/// will be a hash of the input string and is checked against your input
///
/// Syntax: `#[spl_program_error(hash_error_code_start = 1275525928)]`
/// Hash Input: `spl_program_error:<enum name>:<variant name>`
/// Value: `u32::from_le_bytes(<hash of input>[13..17])`
#[proc_macro_attribute]
pub fn spl_program_error(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SplProgramErrorArgs);
    let item_enum = parse_macro_input!(input as ItemEnum);
    MacroType::SplProgramError { args, item_enum }
        .generate_tokens()
        .into()
}
