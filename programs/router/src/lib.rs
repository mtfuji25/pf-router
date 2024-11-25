use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

use crate::instructions::*;

declare_id!("72kmmN8NVxrYCYPdAQ9RGuwXrdTpL54ACERtZAubzUf1");

#[program]
pub mod router {

    use super::*;

    pub fn pf_buy<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PfBuyCtx<'info>>,
        amount: u64,
    ) -> Result<()> {
        pf_buy::process(ctx, amount)
    }

    pub fn pf_sell(ctx: Context<PfSellCtx>, amount: u64) -> Result<()> {
        pf_sell::process(ctx, amount)
    }
}
