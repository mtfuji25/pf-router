use crate::{errors::*, states::BondingCurve};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub fn process(ctx: Context<PfSellCtx>, amount: u64) -> Result<()> {
    require!(amount > 0, RouterError::InvalidAmount);

    // Fetch bondingCurve
    let mut data: &[u8] = &ctx.accounts.bonding_curve.try_borrow_data().unwrap();
    let bonding_curve = BondingCurve::deserialize(&mut data).unwrap();
    msg!("Bonding curve: {:?}", bonding_curve.real_sol_reserves);

    Ok(())
}

#[derive(Accounts)]
pub struct PfSellCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut, 
        token::authority = authority,
        token::mint = mint,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        mut, 
        token::authority = bonding_curve,
        token::mint = mint,
    )]
    pub bonding_curve_ata: Account<'info, TokenAccount>,

    /// CHECK: Pump.fun global PDA
    pub global: UncheckedAccount<'info>,

    /// CHECK: Pump.fun bondingCurve PDA
    #[account(mut)]
    pub bonding_curve: UncheckedAccount<'info>,

    /// CHECK: Pump.fun fee recipient
    pub fee_recipient: UncheckedAccount<'info>,

    /// CHECK: Pump.fun programId
    pub pf_program: UncheckedAccount<'info>,

    /// CHECK: Pump.fun eventAuthority
    #[account(mut)]
    pub event_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
