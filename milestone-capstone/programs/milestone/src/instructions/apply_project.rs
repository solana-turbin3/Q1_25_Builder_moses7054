use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::instruction;

use crate::state::TempTransactionAccount;

// project created --> event emitted
// on dashoboard of ngo it appears. applies -->comes here
// event emitted and updated on companies side
// companies accepts , rejects, closes project.
// close related accounts

#[derive(Accounts)]
#[instruction(project_account_pubkey: Pubkey)]
pub struct ApplyProject<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer= signer, seeds= [b"temp_tx", project_account_pubkey.key().as_ref()], space = TempTransactionAccount::INIT_SPACE, bump)]
    pub temp_transaction_account: Account<'info, TempTransactionAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> ApplyProject<'info> {
    pub fn apply_project(
        &mut self,
        project_account_pubkey: Pubkey,
        submitted_requirements_hash: [u8; 32],
        bumps: &ApplyProjectBumps,
    ) -> Result<()> {
        self.temp_transaction_account
            .set_inner(TempTransactionAccount {
                project_account_pubkey,
                ngo_pubkey: *self.signer.key,
                submitted_requirements_hash,
                status: crate::state::TempTransactionAccountStatus::Processing,
                temp_bump: bumps.temp_transaction_account,
            });
        Ok(())
    }
}
