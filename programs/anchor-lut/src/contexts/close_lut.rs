use anchor_lang::prelude::*;
use solana_address_lookup_table_program::{
    instruction::close_lookup_table,
    ID as ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
};
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct CloseLUT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseLUT<'info> {
    pub fn close_lut(&mut self) -> Result<()> {
        let ix = close_lookup_table(self.lookup_table.key(), self.authority.key(), self.recipient.key());
        
        invoke(&ix,
        &[
            self.lookup_table.to_account_info(),
            self.authority.to_account_info(),
            self.recipient.to_account_info(),
            self.address_lookup_table_program.to_account_info(),
        ])?;

        Ok(())
    }
}

