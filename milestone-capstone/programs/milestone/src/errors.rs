use anchor_lang::error_code;

#[error_code]

pub enum ProjectError {
    #[msg("max project limit reached")]
    MaxProjectsReached,

    #[msg("amount not specified")]
    InvalidAmount,

    #[msg("max ngo application reached")]
    MaxApplicationReached,

    #[msg("Cannot apply. Project closed")]
    ProjectClosed,

    #[msg("This ngo's appliaction is not accepted")]
    ProjectNotAccepted,

    #[msg("wrong project status cannot disburse funds")]
    ProjectStatusWrong,

    #[msg("Insufficient balance in vault to cover fee and payment")]
    InsufficientVaultBalance,

    #[msg("Project not closed")]
    ProjectNotClosed,

    #[msg("This status cannot be used")]
    ProjectStatusNotAllowed,

    #[msg("The status of project account should be NotOpenForApplication for changing the requirements")]
    ProjectWrongStatus,

    #[msg("Ngo application already accepted , cannot change reuirements now")]
    NgoRequirementsNotAllowed,
}
