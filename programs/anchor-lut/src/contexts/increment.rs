use anchor_lang::prelude::*;

use crate::DummyAccount;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"dummy_account"],
        bump = dummy_account.bump,
    )]
    pub dummy_account: Account<'info, DummyAccount>,
}

impl<'info> Increment<'info> {
    pub fn increment(&mut self) -> Result<()>  {
        self.dummy_account.counter += 1;
        
        Ok(())
    }  
}