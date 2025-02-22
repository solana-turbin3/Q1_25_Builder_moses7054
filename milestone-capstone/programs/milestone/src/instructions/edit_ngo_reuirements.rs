use anchor_lang::prelude::*;

use crate::{errors::ProjectError, state::{NgoAccount, TempTransactionAccount, TempTransactionAccountStatus}};

#[derive(Accounts)]
pub struct EditNgoApplication<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account( seeds=[b"ngo",signer.key().as_ref()], bump= ngo.ngo_bump )]
    ngo: Account<'info, NgoAccount>,

    #[account( 
        mut,
        seeds= [b"temp_tx", temp_transaction_account.project_account_pubkey.as_ref(), ngo.key().as_ref()],
        bump= temp_transaction_account.temp_bump
    )]
    pub temp_transaction_account: Account<'info, TempTransactionAccount>,
}

impl <'info>EditNgoApplication<'info>  {
    pub fn edit_ngo_reuirements(&mut self, close: bool, submitted_requirements_hash: Option<[u8; 32]> ) -> Result<()> {
        if close {
            self.temp_transaction_account.close(self.signer.to_account_info())?;

            return Ok(());
        }

        if let Some(submitted_requirements_hash_value) = submitted_requirements_hash {
            require!(self.temp_transaction_account.status != TempTransactionAccountStatus::Accepted, ProjectError::NgoRequirementsNotAllowed );

            self.temp_transaction_account.submitted_requirements_hash = submitted_requirements_hash_value;
        }

        Ok(())
    }
}
