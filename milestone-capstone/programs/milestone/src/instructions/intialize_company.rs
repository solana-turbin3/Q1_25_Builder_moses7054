use anchor_lang::prelude::*;

use crate::state::CompanyAccount;

#[derive(Accounts)]
pub struct InitializeCompany<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"company", signer.key().as_ref()],
        bump,
        space = 8 + CompanyAccount::INIT_SPACE,
    )]
    pub company: Account<'info, CompanyAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeCompany<'info> {
    pub fn intialize_company(
        &mut self,
        name: String,
        business_reg_num: String,

        bumps: &InitializeCompanyBumps,
    ) -> Result<()> {
        self.company.set_inner(CompanyAccount {
            signer: self.signer.key(),
            name,
            company_bump: bumps.company,
            business_reg_num,
            total_projects: 0,
        });

        Ok(())
    }
}
