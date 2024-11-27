use crate::{errors::*, states::BondingCurve};
use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

const SELL_DISCRIMINANT: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct SellArgs {
    amount: u64,
    min_sol_output: u64,
}

pub fn process<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PfSellCtx<'info>>,
    token_amount: u64,
    slippage: u64,
) -> Result<()> {
    require!(token_amount > 0, RouterError::InvalidAmount);

    let mut virtual_sol_reserves = 0;
    let mut virtual_token_reserves = 0;
    let mut real_sol_reserves = 0;

    // Fetch bondingCurve
    {
        let bonding_curve_acc = &ctx.accounts.bonding_curve.to_account_info().clone();
        let data = &bonding_curve_acc.try_borrow_mut_data().unwrap();

        let mut data_slice: &[u8] = &data[8..];
        let bonding_curve = BondingCurve::deserialize(&mut data_slice).unwrap();
        msg!("bonding_curve: {:?}", bonding_curve);

        virtual_sol_reserves = bonding_curve.virtual_sol_reserves as u128;
        virtual_token_reserves = bonding_curve.virtual_token_reserves as u128;
        real_sol_reserves = bonding_curve.real_sol_reserves as u128;
    }

    // Calculate amounts
    let new_token_reserves = virtual_token_reserves
        .checked_add(token_amount as u128)
        .unwrap();
    let virtual_amount = virtual_sol_reserves
        .checked_mul(token_amount as u128)
        .unwrap()
        .checked_div(new_token_reserves)
        .unwrap();

    let sol_amount = (if virtual_amount < real_sol_reserves {
        virtual_amount
    } else {
        real_sol_reserves
    }) as u64;

    let basis_points = 10000;
    let min_sol_output = sol_amount
        .checked_mul(basis_points - slippage)
        .unwrap()
        .checked_div(basis_points)
        .unwrap();
    msg!("amount: {:?}, min_sol_output: {:?}", token_amount, min_sol_output);

    // Prepare args
    let payload = SellArgs {
        amount: token_amount,
        min_sol_output,
    };
    let mut serialized_data = Vec::new();
    payload.serialize(&mut serialized_data)?;
    let mut data = SELL_DISCRIMINANT.to_vec();
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
pub struct PfSellCtx<'info> {
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
