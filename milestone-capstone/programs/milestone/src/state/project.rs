use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProjectAccount {
    pub company_pubkey: Pubkey, // Company's public key
    #[max_len(30)]
    pub project_name: String,
    pub requirements_hash: [u8; 32],  //Hash storying address to db
    pub status: ProjectStatus,        // Project status
    pub max_submissions_allowed: u32, // Maximum allowed submissions
    pub total_submissions: u32,       // Current number of submissions
    pub project_bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, InitSpace, Copy)]
pub enum ProjectStatus {
    Created,            //Project got created
    OpenForApplication, // NGOs can apply for funding
    FundedAndClosed,    // Funding has been granted and closed
    InProgress,         // Project is actively being worked on
    Closed,             // Project is  closed
}
