use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Admin {
    pub admin_signer_pubkey: Pubkey,
    pub max_projects: u32,
    pub fee_basis_points: u16, // basis points 50 = 0.5
    pub admin_bump: u8,
    pub admin_ata: Pubkey,
}

impl Admin {
    pub fn calculate_fee(&self, amount: u64) -> u64 {
        (amount * self.fee_basis_points as u64) / 10_000
    }
}
