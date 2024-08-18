use anchor_lang::prelude::*;
pub mod state;
pub mod contexts;
pub use contexts::*;

declare_id!("2oAPYdwKv92TZr6YELKy4TLXCQxSz16cLzSQ5w7tvFJs");

#[program]
pub mod anchor_amm {
    use super::*;

    // Intialize the pool
    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee:u16) -> Result<()> {
        // save config
        ctx.accounts.save_config(seed, fee, ctx.bumps.config, ctx.bumps.mint_lp)?;
        
        Ok(())
    }

    // Add liquidity to receive LP tokens
    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x:u64, max_y:u64) -> Result<()> {
        // max_x and max_y are used for handling slippage
        // deposit_tokens
        // mint_lp_token
        Ok(())
    }

    // Burn LP tokens to withdraw tokens
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        // burn lp tokens
        // withdraw tokens
        
        Ok(())
    }

    // Swap tokens
    pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64, is_x: bool) -> Result<()> {
        Ok(())
    }

    pub fn lock(ctx: Context<Lock>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn unlock(ctx: Context<Unlock>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn collect(ctx: Context<Collect>) -> Result<()> {
        // collect fees
        Ok(())
    }
}

