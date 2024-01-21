use anchor_lang::prelude::*;
use solana_address_lookup_table_program::{
    instruction::extend_lookup_table,
    ID as ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
};
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct ExtendLUT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub new_address_1: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ExtendLUT<'info> {
    pub fn extend_lookup_table(&mut self,) -> Result<()> {
        let new_addresses: Vec<Pubkey> = vec![self.new_address_1.key()];

        let ix = extend_lookup_table(
            self.lookup_table.key(),
            self.authority.key(),
            Some(self.payer.key()),
            new_addresses,
        );

        invoke(
            &ix,
            &[
                self.lookup_table.to_account_info(),
                self.authority.to_account_info(),
                self.payer.to_account_info(),
                self.system_program.to_account_info(),
                self.address_lookup_table_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}