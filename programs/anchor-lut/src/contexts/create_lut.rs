use anchor_lang::prelude::*;
use solana_address_lookup_table_program::{
    instruction::create_lookup_table_signed,
    ID as ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
};
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct CreateLUT<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateLUT<'info> {
    pub fn create_lookup_table(&mut self, recent_slot: u64) -> Result<()> {
        
        let (ix, lut_key) = create_lookup_table_signed(
            self.authority.key(), 
            self.authority.key(), 
            recent_slot
        );

        match lut_key == self.lookup_table.key() {
            true => {
                invoke(
                    &ix,
                    &[
                        self.lookup_table.to_account_info(),
                        self.authority.to_account_info(),
                        self.system_program.to_account_info(),
                        self.address_lookup_table_program.to_account_info(),
                    ],
                )?;
            },
            false => msg!("The lookup table account is not the expected account"),
        };
        
        Ok(())
    }
}