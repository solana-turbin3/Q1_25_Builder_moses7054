use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub project_key: Pubkey, // Stores the project identifier
    pub vault_ata: Pubkey,   // The associated token account (ATA) for USDC
    pub vault_bump: u8,      // PDA bump
}

impl Space for Vault {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1; // 8-byte discriminator + fields
}
