use anchor_lang::prelude::*;

use crate::DummyAccount;

#[derive(Accounts)]
pub struct CloseState<'info> {
    #[account(mut)]
    receiver: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dummy_account"],
        bump = dummy_account.bump,
        close = receiver,
    )]
    pub dummy_account: Account<'info, DummyAccount>,
}

impl<'info> CloseState<'info> {
    pub fn close_state(&mut self) -> Result<()> {
        msg!("Account closed");

        Ok(())
    }
}