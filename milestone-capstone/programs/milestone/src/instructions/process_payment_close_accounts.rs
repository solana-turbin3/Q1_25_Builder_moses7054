use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{errors::ProjectError, state::{
    Admin, NgoAccount, ProjectAccount, ProjectCompletionDetails, ProjectStatus, TempTransactionAccount, TempTransactionAccountStatus, Vault
}};

#[event]
pub struct  PaymentEvent{
    pub project_account_pubkey: Pubkey,
    pub ngo_account_pubkey: Pubkey,
    pub payment: String
}

// send usdc from vaultAta to ngoAta
// create ngoAta if not there
// close accounts vaultAta, vaultAccount, TempTransactionAccount
// check ngo_pubkey match in ProjectCompletionDetails, check project_account status, check TempTransactionAccount status
// check signer with the company account

// accounts needed - company, project, vaultAccount, vaultAta, NgoAccount, ProjectCompletionDetails, TempTransactionAccount

#[derive(Accounts)]
#[instruction(project_name: String)]
pub struct ProcessPayment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"project",project_account.company_pubkey.as_ref(),project_name.as_bytes() ],
        bump = project_account.project_bump
        )]
    pub project_account: Account<'info, ProjectAccount>,

    #[account(
        mut,
        close = signer,
        seeds = [b"vault", project_account.key().as_ref()],
        bump = vault_account.vault_bump
     )]
    pub vault_account: Account<'info, Vault>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        close = signer,
        associated_token::mint = usdc_mint,
        associated_token::authority = vault_account,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        seeds=[b"ngo",ngo.ngo_signer.as_ref()],
        bump = ngo.ngo_bump 
    )]
    pub ngo: Account<'info, NgoAccount>,

  
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = usdc_mint,
        associated_token::authority = ngosigner_pubkey
    )]
    pub ngo_ata: Account<'info, TokenAccount>,

    #[account(address = ngo.ngo_signer)]
    pub ngosigner_pubkey: SystemAccount<'info>,

    #[account( 
        seeds = [b"project_completion_details",
        project_account.key().as_ref(), ngo.key().as_ref()],
        bump = project_completion_details.completion_bump
    )]
    pub project_completion_details: Account<'info, ProjectCompletionDetails>,

    #[account( 
        mut,
        close = ngosigner_pubkey,
        seeds= [b"temp_tx", project_account.key().as_ref(), ngo.key().as_ref()],
        bump= temp_transaction_account.temp_bump
    )]
    pub temp_transaction_account: Account<'info, TempTransactionAccount>,

    #[account( seeds = [b"admin"],
    bump=admin.admin_bump)]
    pub admin: Account<'info, Admin>,

    #[account(
        associated_token::mint = usdc_mint,
        associated_token::authority = admin.admin_signer_pubkey,
    )]
    pub admin_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ProcessPayment<'info> {
    pub fn make_checks(&mut self) -> Result<()> {

        require!(
            matches!(self.temp_transaction_account.status, TempTransactionAccountStatus::Accepted), ProjectError::ProjectNotAccepted
        );

        require!(
            self.project_account.status == ProjectStatus::Funded,
            ProjectError::ProjectStatusWrong
        );
        
        Ok(())
    }
    

    pub fn payment(&mut self) ->Result<()>{

        self.project_account.status = ProjectStatus::Closed;

        let total_amount = self.vault_ata.amount;
        let fee_amount = self.admin.calculate_fee(total_amount);
        let payment_amount_to_ngo = total_amount - fee_amount;

        // Check if vault_ata has enough balance for both fee and payment
        require!(self.vault_ata.amount >= total_amount, ProjectError::InsufficientVaultBalance);

        // 1. Transfer Fee to Admin Account
        if fee_amount > 0 { // Only transfer fee if it's greater than zero
            let accounts_to_admin_fee_transfer = Transfer {
                from: self.vault_ata.to_account_info(),
                to: self.admin_ata.to_account_info(),
                authority: self.vault_account.to_account_info(),
            };

            let vault_bump = self.vault_account.vault_bump;
            let project_key = self.project_account.key();
            let seeds = &[
                b"vault",
                project_key.as_ref(),
                &[vault_bump],
            ];
            let signer_seeds = &[&seeds[..]];


            let cpi_context_to_admin_fee = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                accounts_to_admin_fee_transfer,
                signer_seeds
            );
            transfer(cpi_context_to_admin_fee, fee_amount)?;
        }


        // 2. Transfer Payment to NGO Account
        if payment_amount_to_ngo > 0 { // Only transfer payment if it's greater than zero
            let accounts_to_ngo_payment_transfer = Transfer {
                from: self.vault_ata.to_account_info(),
                to: self.ngo_ata.to_account_info(),
                authority: self.vault_account.to_account_info(),
            };

            let vault_bump = self.vault_account.vault_bump;
            let project_key = self.project_account.key();
            let seeds = &[
                b"vault",
                project_key.as_ref(),
                &[vault_bump],
            ];
            let signer_seeds = &[&seeds[..]];

            let cpi_context_to_ngo_payment = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                accounts_to_ngo_payment_transfer,
                signer_seeds
            );
            transfer(cpi_context_to_ngo_payment, payment_amount_to_ngo)?;

            emit!(PaymentEvent {
                project_account_pubkey: self.project_account.key(),
                ngo_account_pubkey: self.ngo.key(),
                payment: "payed".to_string(),
            })
        }


        Ok(())
    }

}

