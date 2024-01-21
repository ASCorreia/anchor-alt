use anchor_lang::prelude::*;

#[account]
pub struct DummyAccount {
    pub counter: u8,
    pub bump: u8,
}

impl Space for DummyAccount {
    const INIT_SPACE: usize = 8 + 1 + 1;
}