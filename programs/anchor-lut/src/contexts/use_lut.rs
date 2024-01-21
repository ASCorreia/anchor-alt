use anchor_lang::prelude::*;
use solana_address_lookup_table_program::state::AddressLookupTable;

#[derive(Accounts)]
pub struct UseLUT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,
}

impl<'info> UseLUT<'info> {
    pub fn use_lut(&mut self) -> Result<()> {
        let data_ref = &self.lookup_table.data.borrow();

        let x = AddressLookupTable::deserialize(&data_ref).ok().unwrap();

        match x.meta.authority {
            Some(authority) => {
                msg!("The authority of this Lookup table is {:?}", authority);
            },
            None => msg!("The authority is not set"),
        };

        let lut_pubkeys: Vec<Pubkey> = x.addresses.iter().map(|x| x.clone()).collect();
        
        for val in lut_pubkeys.iter() {
            msg!("The lookup table contains the following pubkey: {:?}", val);
        }

        Ok(())
    }
}