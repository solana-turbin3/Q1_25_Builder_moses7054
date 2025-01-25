use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("Cc3WDyfvWn1LiNtd1Z9a7P3xV4WzsYMQmUy68WVJx24S");

#[program]
pub mod valutt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account{
        init,
        payer=signer,
        space=VaultState::INIT_SPACE+8,
        seeds = [b"state",signer.key().as_ref()],
        bump,
    }]
    pub vault_state: Account<'info, VaultState>,
    #[account{seeds=[vault_state.key().as_ref()],bump}]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account{
       mut,
        seeds = [b"state",signer.key().as_ref()],
        bump = vault_state.state_bump,
    }]
    pub vault_state: Account<'info, VaultState>,
    #[account{seeds=[vault_state.key().as_ref()],bump=vault_state.state_bump}]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(system_program, accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };
        let seeds = &[
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(system_program, accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
