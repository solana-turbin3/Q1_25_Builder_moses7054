use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("DRzTt8VbFsVSvbjWSMNtrDGuDSKVt1AAAtGEtAoPSYvw");

pub use instructions::*;
pub use state::*;

#[program]
pub mod dicegame {
    use super::*;

    pub fn initialize(ctx: Context<Initializ>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, roll: u8, amount: u64) -> Result<()> {
        ctx.accounts.create_bet(&ctx.bumps, seed, roll, amount)?;
        ctx.accounts.deposit(amount)
    }

    pub fn process_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&ctx.bumps, &sig)
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
