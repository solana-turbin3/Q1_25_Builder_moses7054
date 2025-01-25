use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("CTkoHQNZJPpA2uzx8RPog84cEYzEcvYKvmuFSz1qAsMu");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        receive_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .init_escrow_state(seed, receive_amount, ctx.bumps)?;
        ctx.accounts.deposit(deposit_amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
