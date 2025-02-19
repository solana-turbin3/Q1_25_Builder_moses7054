use anchor_lang::error_code;

#[error_code]

pub enum ProjectError {
    #[msg("max project limit reached")]
    MaxProjectsReached,

    #[msg("amount not specified")]
    InvalidAmount,
}
