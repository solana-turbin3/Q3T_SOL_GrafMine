pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod contexts;

use anchor_lang::prelude::*;

pub use constants::*;
pub use state::*;
pub use contexts::*;


declare_id!("GAUiqjw3dS9pXPJydPG4jahZ7UrWEMp8BhhFkqh9H9cJ");

#[program]
pub mod escrow {
    use super::*;

    // pub fn initialize(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
    //     ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
    //     ctx.accounts.deposit(deposit)?;
    //     Ok(())
    // }

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(deposit);
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close();
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }




}
