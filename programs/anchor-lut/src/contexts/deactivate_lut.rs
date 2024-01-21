use anchor_lang::prelude::*;
use solana_address_lookup_table_program::instruction::deactivate_lookup_table;
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct DeactivateLUT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(address = solana_address_lookup_table_program::ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
}

impl<'info> DeactivateLUT<'info> {
    pub fn deactivate_lut(&mut self) -> Result<()> {
        let ix = deactivate_lookup_table(self.lookup_table.key(), self.authority.key());

        invoke(
            &ix,
            &[
                self.lookup_table.to_account_info(),
                self.authority.to_account_info(),
                self.address_lookup_table_program.to_account_info(),
            ],
        )?;
        
        Ok(())
    }
}

