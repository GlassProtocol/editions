pub mod error;
pub mod processor;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use borsh::BorshDeserialize;
use state::{DataV2, PlatformSettings, PrimarySaleReceipt, TokenParameters};
use utils::{
    METADATA_AUTHORITY_PREFIX, MINT_AUTHORITY_PREFIX, PLATFORM_SETTINGS_PREFIX,
    PRIMARY_SALE_RECEIPT_PREFIX, TOKEN_PARAMETERS_PREFIX,
};

declare_id!("GeTddTEvfE8My8HNbnSeS1o2tzyn9Z4S194tCUZPbQ8Y");

#[program]
pub mod editions {
    use super::*;

    pub fn create_platform_settings(
        ctx: Context<CreatePlatformSettings>,
        data: PlatformSettings,
    ) -> Result<()> {
        ctx.accounts.process(data)
    }

    pub fn create_token_parameters(
        ctx: Context<CreateTokenParameters>,
        token_parameters_data: TokenParameters,
        metadata_data: DataV2,
        is_mutable: bool,
    ) -> Result<()> {
        let mint_authority_seeds = [
            MINT_AUTHORITY_PREFIX.as_bytes(),
            &[ctx.bumps["mint_authority"]],
        ];
        ctx.accounts.process(
            token_parameters_data,
            metadata_data,
            is_mutable,
            mint_authority_seeds,
        )
    }

    pub fn update_token_parameters(
        ctx: Context<UpdateTokenParameters>,
        max_supply: Option<u32>,
        primary_sale_token_limit: Option<u32>,
        primary_sale_price: Option<u64>,
    ) -> Result<()> {
        ctx.accounts
            .process(max_supply, primary_sale_token_limit, primary_sale_price)
    }

    pub fn primary_sale<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PrimarySale<'info>>,
        _number_of_sales: u32,
        sale_quantity: u32,
        override_primary_sale_price: Option<u64>,
        override_primary_sale_token_limit: Option<u32>,
    ) -> Result<()> {
        let mint_authority_seeds = [
            MINT_AUTHORITY_PREFIX.as_bytes(),
            &[ctx.bumps["mint_authority"]],
        ];
        ctx.accounts.process(
            sale_quantity,
            ctx.remaining_accounts,
            mint_authority_seeds,
            override_primary_sale_price,
            override_primary_sale_token_limit,
        )
    }
}

// program and program data accounts commented out because they don't work
// with anchor test. Uncomment before deployment. Can be tested on full
// local test validator or devnet.
#[derive(Accounts)]
pub struct CreatePlatformSettings<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, seeds = [PLATFORM_SETTINGS_PREFIX.as_bytes()], bump, payer = payer, space = PlatformSettings::LEN)]
    pub platform_settings: Account<'info, PlatformSettings>,
    // #[account(constraint = program.programdata_address() == Some(program_data.key()))]
    // pub program: Program<'info, crate::program::Mediamarket>,
    // // only the program update authority can initialize the platform settings account
    // #[account(constraint = program_data.upgrade_authority_address == Some(payer.key()))]
    // pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePlatformSettings<'info> {
    #[account(mut, seeds = [PLATFORM_SETTINGS_PREFIX.as_bytes()], bump)]
    pub platform_settings: Account<'info, PlatformSettings>,
    // #[account(constraint = program.programdata_address() == Some(program_data.key()))]
    // pub program: Program<'info, crate::program::Mediamarket>,
    // // only the program update authority can update the platform settings account
    // #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    // pub program_data: Account<'info, ProgramData>,
    pub authority: Signer<'info>,
}

#[derive(Accounts, Clone)]
#[instruction(token_parameters_data: TokenParameters, metadata_data: DataV2)]
pub struct CreateTokenParameters<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, constraint = platform_signer.key() == platform_settings.platform_signer)]
    pub platform_signer: Signer<'info>,
    #[account(init, payer = payer, mint::decimals = 0, mint::authority = mint_authority, mint::freeze_authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    /// CHECK: Created via cpi
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: pubkey checked via seeds
    #[account(seeds = [MINT_AUTHORITY_PREFIX.as_bytes()], bump)]
    pub mint_authority: UncheckedAccount<'info>,
    /// CHECK: pubkey checked via seeds
    #[account(seeds = [METADATA_AUTHORITY_PREFIX.as_bytes()], bump)]
    pub metadata_authority: UncheckedAccount<'info>,
    // Only one series_params account per mint can exist.
    // Platform address has to be the first item in the creators array.
    #[account(init, payer = payer, seeds = [TOKEN_PARAMETERS_PREFIX.as_bytes(), mint.key().as_ref()], bump,
        constraint = token_parameters_data.token_owner == payer.key(),
        constraint = metadata_data.creators.unwrap()[0].address == platform.key(),
        space = TokenParameters::LEN)]
    pub token_parameters: Box<Account<'info, TokenParameters>>,
    /// CHECK: pubkey checked via constraint
    #[account(mut, constraint = platform.key() == platform_settings.platform_signer)]
    pub platform: UncheckedAccount<'info>,
    #[account(seeds = [PLATFORM_SETTINGS_PREFIX.as_bytes()], bump)]
    pub platform_settings: Account<'info, PlatformSettings>,
    pub metadata_program: Program<'info, TokenMetadata>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Clone)]
pub struct UpdateTokenParameters<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, constraint = platform_signer.key() == platform_settings.platform_signer)]
    pub platform_signer: Signer<'info>,
    #[account(mut, constraint = token_parameters.token_owner == payer.key())]
    pub token_parameters: Account<'info, TokenParameters>,
    #[account(seeds = [PLATFORM_SETTINGS_PREFIX.as_bytes()], bump)]
    pub platform_settings: Account<'info, PlatformSettings>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Clone)]
pub struct TransferSol<'info> {
    /// CHECK: unchecked
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: unchecked
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Clone)]
#[instruction(quantity: u32, _number_of_sales: u32)]
pub struct PrimarySale<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, constraint = platform_signer.key() == platform_settings.platform_signer)]
    pub platform_signer: Signer<'info>,
    #[account(mut, constraint = mint.key() == token_parameters.mint)]
    pub mint: Account<'info, Mint>,
    // Only this program will be able to mint tokens.
    #[account(mut, constraint = mint.key() == token_parameters.mint)]
    pub metadata: Box<Account<'info, Metadata>>,
    /// CHECK: pubkey checked via seeds
    #[account(seeds = [MINT_AUTHORITY_PREFIX.as_bytes()], bump)]
    pub mint_authority: UncheckedAccount<'info>,
    // Only one series_params account per mint can exist.
    #[account(
        seeds = [TOKEN_PARAMETERS_PREFIX.as_bytes(), mint.key().as_ref()],
        bump,
        constraint = token_parameters.number_of_sales == _number_of_sales
    )]
    pub token_parameters: Account<'info, TokenParameters>,
    // First creator account has to match platform - this would require updating metadata to include updated platform account
    // if platform_signer settings are ever updated after creation.
    /// CHECK: pubkey checked via seeds
    #[account(
        mut,
        constraint = platform.key() == platform_settings.platform_signer,
        constraint = metadata.data.creators.as_ref().unwrap()[0].address == platform.key()
    )]
    pub platform: UncheckedAccount<'info>,
    #[account(mut, seeds = [PLATFORM_SETTINGS_PREFIX.as_bytes()], bump)]
    pub platform_settings: Account<'info, PlatformSettings>,
    #[account(init_if_needed, payer = payer, associated_token::mint = mint, associated_token::authority = payer)]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: pubkey checked via seeds
    #[account(
        init,
        payer=payer,
        seeds = [PRIMARY_SALE_RECEIPT_PREFIX.as_bytes(), payer.key().as_ref(), mint.key().as_ref(), &_number_of_sales.to_le_bytes()],
        bump,
        space=PrimarySaleReceipt::LEN
    )]
    pub primary_receipt: Account<'info, PrimarySaleReceipt>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Clone)]
pub struct CreateMetaData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: pubkey checked via seeds
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// CHECK: pubkey checked via seeds
    #[account(mut)]
    pub mint_authority: UncheckedAccount<'info>,
    /// CHECK: pubkey checked via seeds
    pub metadata_authority: UncheckedAccount<'info>,
    pub metadata_program: Program<'info, TokenMetadata>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> From<CreateTokenParameters<'info>> for CreateMetaData<'info> {
    fn from(item: CreateTokenParameters<'info>) -> Self {
        CreateMetaData {
            payer: item.payer,
            metadata_account: item.metadata,
            mint: item.mint,
            mint_authority: item.mint_authority,
            metadata_authority: item.metadata_authority,
            metadata_program: item.metadata_program,
            rent: item.rent,
            system_program: item.system_program,
        }
    }
}

#[derive(Clone)]
pub struct TokenMetadata;

impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        mpl_token_metadata::ID
    }
}

#[derive(AnchorDeserialize, Clone, Debug)]
pub struct Metadata(mpl_token_metadata::state::Metadata);

impl Metadata {
    pub const LEN: usize = mpl_token_metadata::state::MAX_METADATA_LEN;
}

impl anchor_lang::AccountDeserialize for Metadata {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        mpl_token_metadata::utils::try_from_slice_checked(
            buf,
            mpl_token_metadata::state::Key::MetadataV1,
            mpl_token_metadata::state::MAX_METADATA_LEN,
        )
        .map_err(Into::into)
    }
}

impl anchor_lang::AccountSerialize for Metadata {}

impl anchor_lang::Owner for Metadata {
    fn owner() -> Pubkey {
        mpl_token_metadata::ID
    }
}

impl core::ops::Deref for Metadata {
    type Target = mpl_token_metadata::state::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
