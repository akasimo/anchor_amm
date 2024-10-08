use anchor_lang::prelude::*;
// use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ Mint, TokenAccount, TokenInterface }
};

use crate::state::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    mint_x: InterfaceAccount<'info, Mint>,
    mint_y: InterfaceAccount<'info, Mint>,

    // /// CHECK: This account is only used to sign. it doesn't contain SOL
    // #[account(seeds = [b"auth"], bump)]
    // pub auth: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"amm".as_ref(), mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, Config>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"mint", config.key().as_ref()],
        bump,
        mint::authority = config,
        mint::decimals = 6,
        mint::freeze_authority = config,
        mint::token_program = token_program,
    )]
    mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        // associated_token::token_program = token_program,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,    
    
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        // associated_token::token_program = token_program,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,

    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}


impl<'info> Initialize<'info> {
    pub fn save_config(&mut self, seed: u64, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.config.set_inner(Config {
            authority: self.admin.key(),
            seed,
            fee,
            locked : false,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            lp_bump: bumps.mint_lp,
            bump: bumps.config,
        });
        require!( fee < 10000, AmmError::InvalidFeeSet);
        Ok(())
    }
}