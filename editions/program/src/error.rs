//! Module provide program defined errors

use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
    #[msg("Invalid primary quantity.")]
    InvalidPrimaryQuantity,
    #[msg(
        "Seller fee basis points not equal to platform fee basis points creator fee basis points."
    )]
    InvalidFeePercentages,
    #[msg("Provided creator accounts do not match Metadata creators.")]
    InvalidCreators,
    #[msg("Semi fungible has an inactive status.")]
    InactiveSemiFungible,
}
