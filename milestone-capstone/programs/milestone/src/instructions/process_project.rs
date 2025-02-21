use anchor_lang::prelude::*;

use crate::state::{
    CompanyAccount, NgoAccount, ProjectAccount, ProjectCompletionDetails, ProjectStatus, Status,
    TempTransactionAccount, TempTransactionAccountStatus,
};

// fund manager role
// after seeing the application of ngo , he either approves or rejects
// from client side -->  projectName, ngoPubkey Accepted or Rejected

// TO DO
// if rejected -> update status project_account, tempTransactionAccount ,
// if accepted -> createProjectCompletionDetails, update project_account, update NgoAccount, TempTransactionAccount.

// accounts needed -> create project_completion_details account, project_account, ngo_account, tempTransactionAccount

#[derive(Accounts)]
#[instruction(project_name: String)]
pub struct ProcessProject<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        has_one = signer,
        seeds = [b"company", signer.key().as_ref()],
        bump= company.company_bump
    )]
    pub company: Account<'info, CompanyAccount>,

    #[account(
        mut,
        seeds = [b"project",project_account.company_pubkey.as_ref(),project_name.as_bytes() ],
        bump = project_account.project_bump

        )]
    pub project_account: Account<'info, ProjectAccount>,

    #[account( seeds=[b"ngo",ngo.ngo_signer.as_ref()], bump = ngo.ngo_bump )]
    pub ngo: Account<'info, NgoAccount>,

    #[account(init, payer = signer, seeds = [b"project_completion_details", project_account.key().as_ref(), ngo.key().as_ref()], space = ProjectCompletionDetails::INIT_SPACE, bump)]
    pub project_completion_details: Account<'info, ProjectCompletionDetails>,

    #[account( 
        mut,
        seeds= [b"temp_tx", project_account.key().as_ref(), ngo.key().as_ref()],
        bump= temp_transaction_account.temp_bump
    )]
    pub temp_transaction_account: Account<'info, TempTransactionAccount>,

    system_program: Program<'info, System>,
}

impl<'info> ProcessProject<'info> {
    pub fn process_project(
        &mut self,
        status: Status,
        merkel_root: Option<[u8; 32]>,
        bumps: &ProcessProjectBumps,
    ) -> Result<()> {
        match status {
            Status::Accepted => {
                self.temp_transaction_account.status = TempTransactionAccountStatus::Accepted;

                self.project_completion_details
                    .set_inner(ProjectCompletionDetails {
                        project_pubkey: self.project_account.key(),
                        ngo_pubkey: self.ngo.key(),
                        merkel_root: merkel_root.expect("Merkel root is required for accepted"),
                        completion_bump: bumps.project_completion_details,
                    });

                self.project_account.status = ProjectStatus::Funded;
            }
            Status::Rejected => {
                self.temp_transaction_account.status = TempTransactionAccountStatus::Rejected;
            }
        }

        Ok(())
    }
}
