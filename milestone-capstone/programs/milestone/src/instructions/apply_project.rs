use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::instruction;

use crate::{
    errors::ProjectError,
    state::{NgoAccount, ProjectAccount, ProjectStatus, TempTransactionAccount},
};

#[event]
pub struct ProjectApplyEvent {
    pub project_account_pubkey: Pubkey,
    pub ngo_account_pubkey: Pubkey,
    pub submitted_requirements_hash: [u8; 32],
}
// project created --> event emitted
// on dashoboard of ngo it appears. applies -->comes here
// inputs: companyPubkey, projectName
// update status on project_account
// event emitted and updated on companies side
// companies accepts , rejects, closes project.
// close related accounts
#[derive(Accounts)]
#[instruction(project_name: String)]
pub struct ApplyProject<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account( seeds=[b"ngo",signer.key().as_ref()], bump= ngo.ngo_bump )]
    ngo: Account<'info, NgoAccount>,

    #[account(
        seeds = [b"project",project_account.company_pubkey.as_ref(),project_name.as_bytes() ],
        bump = project_account.project_bump
        )]
    pub project_account: Account<'info, ProjectAccount>,

    #[account(init, payer= signer, seeds= [b"temp_tx", project_account.key().as_ref(), ngo.key().as_ref()], space = TempTransactionAccount::INIT_SPACE, bump)]
    pub temp_transaction_account: Account<'info, TempTransactionAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> ApplyProject<'info> {
    pub fn apply_project(
        &mut self,
        _project_name: String,
        submitted_requirements_hash: [u8; 32],
        bumps: &ApplyProjectBumps,
    ) -> Result<()> {
        require!(
            self.project_account.total_submissions < self.project_account.max_submissions_allowed,
            ProjectError::MaxApplicationReached
        );
        require!(
            !matches!(
                self.project_account.status,
                ProjectStatus::Closed | ProjectStatus::Funded
            ),
            ProjectError::ProjectClosed
        );

        self.temp_transaction_account
            .set_inner(TempTransactionAccount {
                project_account_pubkey: self.project_account.key(),
                ngo_signer_pubkey: self.signer.key(),
                ngo_account_pubkey: self.ngo.key(),
                submitted_requirements_hash,
                status: crate::state::TempTransactionAccountStatus::Processing,
                temp_bump: bumps.temp_transaction_account,
            });
        self.project_account.total_submissions += 1;

        emit!(ProjectApplyEvent {
            project_account_pubkey: self.project_account.key(),
            ngo_account_pubkey: self.signer.key(),
            submitted_requirements_hash: submitted_requirements_hash
        });

        Ok(())
    }
}
