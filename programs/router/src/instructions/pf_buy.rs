use crate::{errors::*, states::BondingCurve};
use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

const BUY_DISCRIMINANT: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct BuyArgs {
    amount: u64,
    max_sol_cost: u64,
}

pub fn process<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PfBuyCtx<'info>>,
    sol_amount: u64,
) -> Result<()> {
    require!(sol_amount > 0, RouterError::InvalidAmount);

    let mut virtual_sol_reserves = 0;
    let mut virtual_token_reserves = 0;
    let mut real_token_reserves = 0;

    // Fetch bondingCurve
    {
        let bonding_curve_acc = &ctx.accounts.bonding_curve.to_account_info().clone();
        let data = &bonding_curve_acc.try_borrow_mut_data().unwrap();

        let mut data_slice: &[u8] = &data[8..];
        let bonding_curve = BondingCurve::deserialize(&mut data_slice).unwrap();
        msg!("bonding_curve: {:?}", bonding_curve);

        virtual_sol_reserves = bonding_curve.virtual_sol_reserves as u128;
        virtual_token_reserves = bonding_curve.virtual_token_reserves as u128;
        real_token_reserves = bonding_curve.real_token_reserves as u128;
    }

    // Calculate amounts
    let new_sol_reserves = virtual_sol_reserves
        .checked_add(sol_amount as u128)
        .unwrap();
    let new_token_reserves = virtual_token_reserves
        .checked_mul(sol_amount as u128)
        .unwrap()
        .checked_div(new_sol_reserves)
        .unwrap();
    let virtual_amount = virtual_token_reserves
        .checked_sub(new_token_reserves)
        .unwrap();

    // let amount = (if virtual_amount < real_token_reserves {
    //     virtual_amount
    // } else {
    //     real_token_reserves
    // }) as u64;
    let amount = virtual_amount as u64;
    let max_sol_cost = sol_amount;
    msg!("amount: {:?}, max_sol_cost: {:?}", amount, max_sol_cost);

    // Prepare args
    let payload = BuyArgs {
        amount,
        max_sol_cost,
    };
    let mut serialized_data = Vec::new();
    payload.serialize(&mut serialized_data)?;
    let mut data = BUY_DISCRIMINANT.to_vec();
    data.append(&mut serialized_data);

    // CPI call
    {
        let account_metas = vec![
            AccountMeta::new_readonly(ctx.accounts.global.key(), false),
            AccountMeta::new(ctx.accounts.fee_recipient.key(), false),
            AccountMeta::new_readonly(ctx.accounts.mint.key(), false),
            AccountMeta::new(ctx.accounts.bonding_curve.key(), false),
            AccountMeta::new(ctx.accounts.bonding_curve_ata.key(), false),
            AccountMeta::new(ctx.accounts.user_ata.key(), false),
            AccountMeta::new(ctx.accounts.authority.key(), true),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
            AccountMeta::new_readonly(ctx.accounts.event_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.pf_program.key(), false),
        ];

        let ix = Instruction::new_with_bytes(
            ctx.accounts.pf_program.key(),
            &data,
            account_metas.clone(),
        );
        invoke(
            &ix,
            &[
                ctx.accounts.global.to_account_info().clone(),
                ctx.accounts.fee_recipient.to_account_info().clone(),
                ctx.accounts.mint.to_account_info().clone(),
                ctx.accounts.bonding_curve.to_account_info().clone(),
                ctx.accounts.bonding_curve_ata.to_account_info().clone(),
                ctx.accounts.user_ata.to_account_info().clone(),
                ctx.accounts.authority.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
                ctx.accounts.token_program.to_account_info().clone(),
                ctx.accounts.rent.to_account_info().clone(),
                ctx.accounts.event_authority.to_account_info().clone(),
            ],
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct PfBuyCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::authority = authority,
        token::mint = mint,
    )]
    pub user_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        token::authority = bonding_curve,
        token::mint = mint,
    )]
    pub bonding_curve_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK: Pump.fun global PDA
    pub global: UncheckedAccount<'info>,

    /// CHECK: Pump.fun bondingCurve PDA
    #[account(mut)]
    pub bonding_curve: UncheckedAccount<'info>,

    /// CHECK: Pump.fun fee recipient
    #[account(mut)]
    pub fee_recipient: UncheckedAccount<'info>,

    /// CHECK: Pump.fun programId
    pub pf_program: UncheckedAccount<'info>,

    /// CHECK: Pump.fun eventAuthority
    #[account(mut)]
    pub event_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,
}
