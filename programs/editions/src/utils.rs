use crate::{error::ProgramError, id, CreateMetaData, TransferSol};
use anchor_lang::prelude::*;
use mpl_token_metadata::state::{Creator, DataV2};

pub const PLATFORM_SETTINGS_PREFIX: &str = "platform_settings";
pub const MINT_AUTHORITY_PREFIX: &str = "mint_authority";
pub const METADATA_AUTHORITY_PREFIX: &str = "metadata_authority";
pub const TOKEN_PARAMETERS_PREFIX: &str = "token_parameters";
pub const PRIMARY_SALE_RECEIPT_PREFIX: &str = "primary_sale_receipt";

pub fn transfer_sol<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, TransferSol<'info>>,
    lamports: u64,
) -> Result<()> {
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key(),
        &ctx.accounts.to.key(),
        lamports,
    );
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.to.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn create_metadata_accounts_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMetaData<'info>>,
    update_authority_is_signer: bool,
    is_mutable: bool,
    data: DataV2,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::create_metadata_accounts_v2(
        mpl_token_metadata::ID.clone(),
        ctx.accounts.metadata_account.to_account_info().key(),
        ctx.accounts.mint.to_account_info().key(),
        ctx.accounts.mint_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.metadata_authority.key(),
        data.name,
        data.symbol,
        data.uri,
        data.creators,
        data.seller_fee_basis_points,
        update_authority_is_signer,
        is_mutable,
        data.collection,
        data.uses,
    );
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata_authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn assert_creator_shares_correct(
    creators: Vec<Creator>,
    seller_fee_basis_points: u16,
    platform_fee_basis_points: u16,
    creator_fee_basis_points: u16,
) -> Result<()> {
    let creators_sum = creators.iter().fold(0, |mut acc, c| {
        acc = acc + c.share;
        acc
    });

    msg!(
        "creators_share: {}",
        platform_fee_basis_points * 100 / seller_fee_basis_points
    );

    if (creators_sum != 100)
        || ((platform_fee_basis_points + creator_fee_basis_points) != seller_fee_basis_points)
        || (creators[0].share as u16 != platform_fee_basis_points * 100 / seller_fee_basis_points)
        || ((100 - creators[0].share) as u16
            == creator_fee_basis_points * 100 / seller_fee_basis_points)
    {
        return Err(error!(ProgramError::InvalidFeePercentages));
    }

    Ok(())
}

pub fn find_primary_receipt_address(
    buyer: &Pubkey,
    mint: &Pubkey,
    primary_sale_count: u32,
) -> (Pubkey, u8) {
    let primary_receipt_seeds = &[
        PRIMARY_SALE_RECEIPT_PREFIX.as_bytes(),
        buyer.as_ref(),
        mint.as_ref(),
        &primary_sale_count.to_le_bytes(),
    ];
    Pubkey::find_program_address(primary_receipt_seeds, &id())
}
