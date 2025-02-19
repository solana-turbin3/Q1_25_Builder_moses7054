use anchor_lang::prelude::*;

// PDA Seed: [ "company", signerPubkey ]

#[account]
#[derive(InitSpace)] // Auto-calculates storage size
pub struct CompanyAccount {
    pub signer: Pubkey,
    #[max_len(40)]
    pub name: String, // Company name (max 54 chars)

    #[max_len(24)]
    pub business_reg_num: String, // Business registration number (max 24 chars)

    pub company_bump: u8,    // PDA bump seed
    pub max_projects: u32,   // Max allowed projects
    pub total_projects: u32, // Current project count
}
