use anchor_lang::prelude::*;

#[derive(account)]
#[Instruction(seeds:u64)]

pub struct Make<'info> {
    #[account{mut}]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint_a,associated_token::authority=maker)]
    pub maker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer=maker,
        space=[b"escrow",maker.key.as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account(init,associated_token::mint_a,associated_token::authority,payer=maker)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Make<'info> {
    pub fn init_escow_state(
        &mut self,
        seed: u64,
        recieve_amount: u64,
        bumps: MakeBumps,
    ) -> Result<()> {
        self.escrow.set_inner(EscrowState {
            recieve_amount,
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            bump: bump.escrow,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.maker_mint_a_ata.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;

        Ok(())
    }
}
