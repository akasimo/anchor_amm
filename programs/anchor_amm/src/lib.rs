use anchor_lang::prelude::*;

pub mod state;
pub use state::*;
pub mod contexts;
pub use contexts::*;

pub mod errors;
mod helpers;

declare_id!("2oAPYdwKv92TZr6YELKy4TLXCQxSz16cLzSQ5w7tvFJs");

#[program]
pub mod anchor_amm {
    use super::*;

    // Intialize the pool
    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee:u16) -> Result<()> {
        // save config
        ctx.accounts.save_config(seed, fee, &ctx.bumps)
    }

    // Add liquidity to receive LP tokens
    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x:u64, max_y:u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    // Burn LP tokens to withdraw tokens
    pub fn withdraw(ctx: Context<Withdraw>, amount:u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)
    }

    pub fn swap(ctx: Context<Swap>, mint_deposit:Pubkey, amount_in: u64, amount_out_min: u64) -> Result<()> {
        ctx.accounts.swap(mint_deposit, amount_in, amount_out_min)
    }

    pub fn lock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.lock()
    }

    pub fn unlock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.unlock()
    }
}

