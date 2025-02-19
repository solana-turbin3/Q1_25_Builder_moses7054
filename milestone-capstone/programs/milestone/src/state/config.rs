use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub max_projects: u32,
    pub fee_basis_points: u16, // basis points 50 = 0.5
}

impl Config {
    pub fn calculate_fee(&self, amount: u64) -> u64 {
        (amount * self.fee_basis_points as u64) / 10_000
    }
}
