use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Not allowed authority")]
    NotAllowedAuthority,

    #[msg("Invalid Tier")]
    InvalidTier,

    #[msg("Should be over minimum amount")]
    InsufficientAmount,

    #[msg("Incorrect User State")]
    IncorrectUserState,
}
