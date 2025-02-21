use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::Admin;

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"admin"],
        bump,
        space = 8 + Admin::INIT_SPACE,
    )]
    pub admin: Account<'info, Admin>,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint = usdc_mint,
        associated_token::authority = signer,
    )]
    pub admin_ata: Account<'info, TokenAccount>, // admin ata , storying usdc

    pub usdc_mint: Account<'info, Mint>, // mint of usdc

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAdmin<'info> {
    pub fn intialize_admin(
        &mut self,
        max_projects: u32,
        fee_basis_points: u16,
        bumps: &InitializeAdminBumps,
    ) -> Result<()> {
        self.admin.set_inner(Admin {
            admin_signer_pubkey: self.signer.key(),
            max_projects,
            fee_basis_points,
            admin_bump: bumps.admin,
            admin_ata: self.admin_ata.key(),
        });
        Ok(())
    }
}
