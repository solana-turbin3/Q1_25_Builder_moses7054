use anchor_lang::{prelude::*, system_program::Transfer};

use crate::{
    errors::ProjectError,
    state::{CompanyAccount, ProjectAccount, ProjectStatus},
};

// TODO test

#[derive(Accounts)]
#[instruction(project_name: String)]
pub struct EditProject<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        mut,
        has_one = signer,
        seeds = [b"company", signer.key().as_ref()],
        bump = company.company_bump
    )]
    pub company: Account<'info, CompanyAccount>,

    #[account(
        mut,
        seeds = [b"project",project_account.company_pubkey.as_ref(),project_name.as_bytes() ],
        bump = project_account.project_bump
        )]
    pub project_account: Account<'info, ProjectAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> EditProject<'info> {
    pub fn edit_project_account(
        &mut self,
        status: Option<ProjectStatus>,
        requirements_hash: Option<[u8; 32]>,
        close: bool,
    ) -> Result<()> {
        if let Some(status_value) = status {
            require!(
                matches!(
                    status_value,
                    ProjectStatus::OpenForApplication | ProjectStatus::NotOpenForApplication
                ),
                ProjectError::ProjectStatusNotAllowed
            );

            self.project_account.status = status_value;

            return Ok(());
        }

        if let Some(requirements_hash_value) = requirements_hash {
            require!(
                self.project_account.status == ProjectStatus::NotOpenForApplication,
                ProjectError::ProjectWrongStatus
            );
            self.project_account.requirements_hash = requirements_hash_value;

            return Ok(());
        }

        if close {
            self.project_account.close(self.signer.to_account_info())?;
        }

        Ok(())
    }
}
