use anchor_lang::prelude::*;

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
            admin_pubkey: self.signer.key(),
            max_projects,
            fee_basis_points,
            config_bump: bumps.admin,
        });
        Ok(())
    }
}
