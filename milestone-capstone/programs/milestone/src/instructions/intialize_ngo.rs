use anchor_lang::prelude::*;

use crate::state::NgoAccount;

#[derive(Accounts)]
pub struct InitializeNgo<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(init, payer = signer, seeds=[b"ngo",signer.key().as_ref()], space = 8 + NgoAccount::INIT_SPACE, bump )]
    ngo: Account<'info, NgoAccount>,

    system_program: Program<'info, System>,
}

impl<'info> InitializeNgo<'info> {
    pub fn initialize_ngo(&mut self, name: String, bumps: &InitializeNgoBumps) -> Result<()> {
        self.ngo.set_inner(NgoAccount {
            name,
            completed_projects: 0,
            merkel_root: None,
            ngo_bump: bumps.ngo,
        });
        Ok(())
    }
}
