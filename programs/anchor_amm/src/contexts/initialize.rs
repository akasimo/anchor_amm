use anchor_lang::prelude::*;
use anchor_spl::{token::TokenAccount, token_interface::{Mint, TokenInterface}};

use crate::state::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        token_program::mint = mint_x,
        token_program::authority = config,
    )]
    mint_x: InterfaceAccount<'info, Mint>,
    mint_y: InterfaceAccount<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer=signer,
        mint::authority = config,
        mint::freeze_authority = config,
        mint::decimals = 6,
        mint::token_program = token_program,
        seeds = [b"mint".as_ref(), config.key().as_ref()],
        bump
    )]
    mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=signer,
        space = 8 + Config::INITSPACE,
        seeds = [b"amm".as_ref(), mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, Config>,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,    
    
    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,

    associated_token_program: Interface<'info, TokenInterface>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}


impl<'info> Initialize<'info> {
    pub fn save_config(&mut self, seed: u64, fee: u16, bump:u8, lp_bump: u8) -> ProgramResult {
        self.config.set_inner(Config {
            seed,
            fee,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            lp_bump,
            bump,
        });
        Ok(())
    }
}