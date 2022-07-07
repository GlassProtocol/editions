use anchor_lang::prelude::*;
use mpl_token_metadata::state::{
    Collection as CollectionMpl, Creator as CreatorMpl, DataV2 as DataV2Mpl,
    UseMethod as UseMethodMpl, Uses as UsesMpl,
};

//==============================
// PlatformSettings
// replacing AdminSettings
// platform signer controls creation, updating, and primary sales
//==============================

#[account]
#[derive(Default)]
pub struct PlatformSettings {
    pub platform_treasury: Pubkey,
    pub platform_signer: Pubkey,
}

impl PlatformSettings {
    pub const LEN: usize = 8
    + 32    // platform_treasury
    + 32;   // platform_signer
}

//==============================
// TokenParameters
// replace Semifungible
//==============================

#[account]
#[derive(PartialEq, Debug, Default, Copy)]
pub struct TokenParameters {
    pub token_owner: Pubkey,
    pub mint: Pubkey,
    pub metadata: Pubkey,
    pub primary_sale_price: u64, // in lamports
    pub max_supply: u32,
    pub primary_sale_token_limit: u32,
    pub number_of_sales: u32,
    pub platform_fee_basis_points: u16,
    pub creator_fee_basis_points: u16,
}

impl TokenParameters {
    pub const LEN: usize = 8
    + 32    // token_owner
    + 32    // mint
    + 32    // metadata
    + 8     // primary_sale_price
    + 4     // max_supply
    + 4     // primary_sale_token_limit
    + 4     // number_of_sales
    + 2     // platform_fee_basis_points
    + 2;     // creator_fee_basis_points


    pub fn primary_primary_sale_price(&self, quantity: u64) -> u64 {
        quantity * self.primary_sale_price
    }

    pub fn total_fee(&self, price: u64) -> u64 {
        price * (self.platform_fee_basis_points + self.creator_fee_basis_points) as u64 / 10_000
    }
}

//==============================
// Primary Sale Receipt
//==============================

#[account]
#[derive(Default)]
pub struct PrimarySaleReceipt {
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub primary_sale_price: u64,
    pub sale_quantity: u64,
    pub created_at: i64,
}

impl PrimarySaleReceipt {
    pub const LEN: usize = 8
    + 32    // buyer
    + 32    // mint 
    + 8     // primary_sale_price
    + 8     // sale_quantity
    + 8;    // created_at
}


//==============================
// Metadata
//==============================

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct DataV2 {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
}

impl From<DataV2> for DataV2Mpl {
    fn from(item: DataV2) -> Self {
        DataV2Mpl {
            name: item.name,
            symbol: item.symbol,
            uri: item.uri,
            seller_fee_basis_points: item.seller_fee_basis_points,
            creators: item
                .creators
                .map(|a| a.into_iter().map(|v| v.into()).collect()),
            collection: item.collection.map(|v| v.into()),
            uses: item.uses.map(|v| v.into()),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

impl From<Creator> for CreatorMpl {
    fn from(item: Creator) -> Self {
        CreatorMpl {
            address: item.address,
            verified: item.verified,
            share: item.share,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

impl From<Uses> for UsesMpl {
    fn from(item: Uses) -> Self {
        UsesMpl {
            use_method: item.use_method.into(),
            remaining: item.remaining,
            total: item.total,
        }
    }
}

impl From<UseMethod> for UseMethodMpl {
    fn from(item: UseMethod) -> Self {
        match item {
            UseMethod::Burn => UseMethodMpl::Burn,
            UseMethod::Multiple => UseMethodMpl::Multiple,
            UseMethod::Single => UseMethodMpl::Single,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

impl From<Collection> for CollectionMpl {
    fn from(item: Collection) -> Self {
        CollectionMpl {
            verified: item.verified,
            key: item.key,
        }
    }
}
