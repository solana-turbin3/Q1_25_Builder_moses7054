use anchor_lang::prelude::*;

mod instructions;

mod errors;
mod state;

use instructions::*;

declare_id!("3ekfBBpPEzHCe3Z9DPE9wp7Hgx82LoCGMWS83ez6Ctnj");

#[program]
pub mod milestone {

    use super::*;

    pub fn init_admin(
        ctx: Context<InitializeAdmin>,
        max_projects: u32,
        fee_basis_points: u16,
    ) -> Result<()> {
        ctx.accounts
            .intialize_admin(max_projects, fee_basis_points, &ctx.bumps)?;
        Ok(())
    }

    pub fn init_company(
        ctx: Context<InitializeCompany>,
        name: String,
        business_reg_num: String,
    ) -> Result<()> {
        ctx.accounts
            .intialize_company(name, business_reg_num, &ctx.bumps)?;

        Ok(())
    }

    pub fn create_project(
        ctx: Context<InitializeProject>,
        project_name: String,
        requirements_hash: [u8; 32],
        max_submissions_allowed: u32,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.verify_and_create_project(
            project_name,
            requirements_hash,
            max_submissions_allowed,
            &ctx.bumps,
        )?;

        ctx.accounts.deposit(amount, &ctx.bumps)?;

        Ok(())
    }

    pub fn init_ngo(ctx: Context<InitializeNgo>, name: String) -> Result<()> {
        ctx.accounts.initialize_ngo(name, &ctx.bumps)?;
        Ok(())
    }

    pub fn initiate_project(
        ctx: Context<ApplyProject>,
        project_account_pubkey: Pubkey,
        submitted_requirements_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.apply_project(
            project_account_pubkey,
            submitted_requirements_hash,
            &ctx.bumps,
        )?;
        Ok(())
    }
}
