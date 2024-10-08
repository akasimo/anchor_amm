use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{TransferChecked, transfer_checked}, 
    token_interface::{ Mint, TokenAccount, TokenInterface}
};
use crate::{assert_not_locked, state::Config};
use crate::errors::AmmError;
use crate::assert_non_zero;

use constant_product_curve::{ConstantProduct, LiquidityPair};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Swap<'info> {
    #[account(mut)]
    user: Signer<'info>,

    mint_x: InterfaceAccount<'info, Mint>,
    mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    user_ata_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    user_ata_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"mint", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"amm".as_ref(), mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = config.bump,
    )]
    config: Account<'info, Config>,

    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl <'info> Swap<'info> {
    pub fn swap(&mut self, mint_deposit:Pubkey, amount_in: u64, amount_out_min: u64) -> Result<()> {
        assert_not_locked!(self);
        assert_non_zero!([amount_in, amount_out_min]);

        let mut curve = ConstantProduct::init(
            self.vault_x.amount, 
            self.vault_y.amount, 
            self.mint_lp.supply, 
            self.config.fee, 
            None)
            .map_err(AmmError::from)?;

        let (p, mint_withdraw) = match mint_deposit {
            m if m == self.mint_x.key() => (LiquidityPair::X, self.mint_y.key()),
            m if m == self.mint_y.key() => (LiquidityPair::Y, self.mint_x.key()),
            _ => return Err(AmmError::InvalidInputMint.into())
        };

        let res = curve.swap(p, amount_in, amount_out_min).map_err(AmmError::from)?;

        assert_non_zero!([res.deposit, res.withdraw]);

        self.deposit_token(mint_deposit, res.deposit)?;
        self.withdraw_token(mint_withdraw, res.withdraw)?;
        Ok(())
        
    }

    pub fn deposit_token(
        &mut self,
        mint_deposit: Pubkey,
        amount:u64,
    ) -> Result<()> {

        let mint;
        let (from, to) = match mint_deposit {
            m if m == self.mint_x.key() => {
                mint = self.mint_x.clone();
                (self.user_ata_x.to_account_info(), self.vault_x.to_account_info())
            },
            m if m == self.mint_y.key() => {
                mint = self.mint_y.clone();
                (self.user_ata_y.to_account_info(), self.vault_y.to_account_info())
            },
            _ => return Err(AmmError::InvalidInputMint.into())
        };

        let account = TransferChecked {
            from,
            mint: mint.to_account_info(),
            to,
            authority: self.user.to_account_info()
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), account);

        transfer_checked(ctx, amount, 6)
    }

    pub fn withdraw_token(
        &mut self,
        mint_withdraw:Pubkey,
        amount:u64
    ) -> Result<()> {

        let mint;
        let (from, to) = match mint_withdraw {
            m if m == self.mint_x.key() => {
                mint = self.mint_x.clone();
                (self.vault_x.to_account_info(), self.user_ata_x.to_account_info())
            },
            m if m == self.mint_y.key() => {
                mint = self.mint_y.clone();
                (self.vault_y.to_account_info(), self.user_ata_y.to_account_info())
            },
            _ => return Err(AmmError::InvalidInputMint.into())
        };

        let account = TransferChecked{
            from,
            mint: mint.to_account_info(),
            to,
            authority: self.config.to_account_info()
        };

        let binding_mint_x = self.mint_x.to_account_info().key();
        let binding_mint_y = self.mint_y.to_account_info().key();
        let binding_seed = self.config.seed.to_le_bytes();
        let seeds: &[&[u8]; 5] = &[
            &b"amm"[..],
            &binding_mint_x.as_ref(),
            &binding_mint_y.as_ref(),
            &binding_seed.as_ref(),
            &[self.config.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), account, signer_seeds);
        transfer_checked(ctx, amount, 6)

    }

}