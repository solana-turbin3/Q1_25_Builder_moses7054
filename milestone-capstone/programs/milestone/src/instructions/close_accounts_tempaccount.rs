use anchor_lang::prelude::*;


use crate::{errors::ProjectError, state::{ProjectAccount, ProjectStatus, TempTransactionAccount}};

// accounts needed -> project_account, temptransactionaccount,
// check project_account - status is Funded or closed
// then close temp_transaction_account

#[derive(Accounts)]
pub struct CloseAccountTemp<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        seeds = [b"project",project_account.company_pubkey.as_ref(),project_account.project_name.as_bytes() ],
        bump= project_account.project_bump,
        )]
    pub project_account: Account<'info, ProjectAccount>,

    #[account( 
        mut,
        close = signer,
        seeds= [b"temp_tx", project_account.key().as_ref(), temp_transaction_account.ngo_account_pubkey.key().as_ref()],
        bump= temp_transaction_account.temp_bump
    )]
    pub temp_transaction_account: Account<'info, TempTransactionAccount>,
}


impl <'info> CloseAccountTemp<'info> {

    pub fn close_accounts(&mut self) ->Result<()>{
        
        require!(self.project_account.status == ProjectStatus::Closed, ProjectError::ProjectNotClosed );

        Ok(())
    }
    
}