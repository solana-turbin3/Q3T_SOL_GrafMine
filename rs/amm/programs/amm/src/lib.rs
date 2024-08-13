pub mod state;
pub mod contexts;
pub use contexts::*;
use anchor_lang::prelude::*;

declare_id!("DD4fn46froMKiX6GCDcdMbBuERexQXVixiUd1dMqe2eA");

#[program]
pub mod amm {
    use super::*;

    // Initialize an amm
    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, amount_x:u64, amount_y:u64) -> Result<()> {
        ctx.accounts.save_config(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp)?;
        ctx.accounts.deposit(amount_x, true)?;
        ctx.accounts.deposit(amount_y, false)?;
        ctx.accounts.mint_lp_tokens(amount_x, amount_y)?;

        Ok(())
    }

    // // Deposit liquidity to mint LP tokens
    // pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x : u64, max_y: u64) -> Result<()> {
    //
    //
    //     Ok(())
    // }
    //
    // // burn LP tokens to withdraw liquidity
    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x : u64, min_y: u64) -> Result<()> {
    //
    //
    //     Ok(())
    // }
    //
    //
    // pub fn swap(ctx: Context<Swap>, amount: u64, min_receive : u64, is_x: bool, expiration: i64) -> Result<()> {
    //
    //
    //     Ok(())
    // }
}
