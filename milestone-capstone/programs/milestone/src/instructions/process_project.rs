use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ProcessProject<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}
