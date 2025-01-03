use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

use crate::instructions::*;

declare_id!("74EXxDQkbybAivua2WKhVcHSmeyiXp47mmqVwGT5dc2s");

#[program]
pub mod router {

    use super::*;

    pub fn pf_buy<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PfBuyCtx<'info>>,
        amount: u64,
        slippage: u64,
    ) -> Result<()> {
        pf_buy::process(ctx, amount, slippage)
    }

    pub fn pf_sell<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PfSellCtx<'info>>,
        amount: u64,
        slippage: u64,
    ) -> Result<()> {
        pf_sell::process(ctx, amount, slippage)
    }
}
