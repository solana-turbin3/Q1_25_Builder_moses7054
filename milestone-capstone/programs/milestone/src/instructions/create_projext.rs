use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

// check with project max in company_account - done
// initialize project account - done
// create vault and transfer usdc - done
// updtae total projects in company_Account - done

use crate::{
    errors::ProjectError,
    state::{Admin, CompanyAccount, ProjectAccount, ProjectStatus, Vault},
};

#[event]
pub struct ProjectCreationEvent {
    pub company_pubkey: Pubkey, // Company's public key
    pub project_name: String,
    pub requirements_hash: [u8; 32],
}

#[derive(Accounts)]
#[instruction(project_name: String )]
pub struct InitializeProject<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, //signer , creating project

    #[account(
        mut,
        has_one = signer,
        seeds = [b"company", signer.key().as_ref()],
        bump = company.company_bump
    )]
    pub company: Account<'info, CompanyAccount>, // to check company account exist or not

    #[account(init,
    payer= signer,
    seeds = [b"project",company.key().as_ref(),project_name.as_bytes() ],
    space = 8 + ProjectAccount::INIT_SPACE, //using drive space
    bump
    )]
    pub project_account: Account<'info, ProjectAccount>, // project account

    #[account(init,
        payer = signer,
        seeds = [b"vault", project_account.key().as_ref()],
        space =  Vault::INIT_SPACE, // manually getting space so 8 is already considered
        bump
     )]
    pub vault_account: Account<'info, Vault>, //controls vault ata

    #[account(
        init,
        payer = signer,
        associated_token::mint = usdc_mint,
        associated_token::authority = vault_account,
    )]
    pub vault_ata: Account<'info, TokenAccount>, // Stores usdc

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = signer,
    )]
    pub signer_ata: Account<'info, TokenAccount>, // signers ata , storying usdc

    pub usdc_mint: Account<'info, Mint>, // mint of usdc

    #[account( seeds = [b"admin"],
    bump=admin.admin_bump)]
    pub admin: Account<'info, Admin>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProject<'info> {
    pub fn verify_and_create_project(
        &mut self,
        project_name: String,
        requirements_hash: [u8; 32],
        max_submissions_allowed: u32,
        bumps: &InitializeProjectBumps,
    ) -> Result<()> {
        require!(
            self.company.total_projects < self.admin.max_projects,
            ProjectError::MaxProjectsReached
        );
        self.project_account.set_inner(ProjectAccount {
            company_pubkey: self.company.key(),
            project_name: project_name.clone(),
            requirements_hash,
            status: ProjectStatus::OpenForApplication,
            max_submissions_allowed,
            total_submissions: 0,
            project_bump: bumps.project_account,
        });

        self.company.total_projects += 1; // updating company account

        emit!(ProjectCreationEvent {
            company_pubkey: self.company.key(),
            project_name: project_name,
            requirements_hash: requirements_hash
        });

        Ok(())
    }

    pub fn deposit(&mut self, amount: u64, bumps: &InitializeProjectBumps) -> Result<()> {
        require!(amount != 0, ProjectError::InvalidAmount);

        self.vault_account.set_inner(Vault {
            project_key: self.project_account.key(),
            vault_ata: self.vault_ata.key(),
            vault_bump: bumps.vault_account,
        });

        let from = self.signer_ata.to_account_info();
        let to = self.vault_ata.to_account_info();

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.signer.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(ctx, amount)?;

        self.project_account.status = ProjectStatus::OpenForApplication;

        Ok(())
    }
}
