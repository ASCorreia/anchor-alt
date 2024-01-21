use anchor_lang::prelude::*;

use crate::DummyAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [b"dummy_account"],
        bump,
        payer = signer,
        space = DummyAccount::INIT_SPACE,
    )]
    pub dummy_account: Account<'info, DummyAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.dummy_account.bump = bumps.dummy_account;
        self.dummy_account.counter = 0;

        Ok(())
    }

    pub fn print_counter(&mut self) -> Result<()> {
        msg!("The counter is {}", self.dummy_account.counter);

        Ok(())
    }
}