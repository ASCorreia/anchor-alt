use anchor_lang::prelude::*;

declare_id!("387hciuQ436coWhx5QFBAKihMSL1PeQCYPAiDGvrVcMX");

pub mod contexts;
pub mod state;

pub use contexts::*;
pub use state::*;

#[program]
pub mod anchor_lut {
    use crate::contexts::CreateLUT;

    use super::*;

    pub fn initialize_state(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn print_counter(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.print_counter()
    }

    pub fn increment_counter(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment()
    }

    pub fn close_state(ctx: Context<CloseState>) -> Result<()> {
        ctx.accounts.close_state()
    }

    pub fn initialize(ctx: Context<CreateLUT>, recent_slot: u64) -> Result<()> {
        ctx.accounts.create_lookup_table(recent_slot)
    }

    pub fn extend_lut(ctx: Context<ExtendLUT>) -> Result<()> {
        ctx.accounts.extend_lookup_table()
    }

    pub fn deactivate_lut(ctx: Context<DeactivateLUT>) -> Result<()> {
        ctx.accounts.deactivate_lut()
    }

    pub fn close_lut(ctx: Context<CloseLUT>) -> Result<()> {
        ctx.accounts.close_lut()
    }

    pub fn use_lut(ctx: Context<UseLUT>) -> Result<()> {
        ctx.accounts.use_lut()
    }
}
