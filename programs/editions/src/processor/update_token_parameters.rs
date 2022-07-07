use crate::{UpdateTokenParameters};
use anchor_lang::prelude::*;

impl<'info> UpdateTokenParameters<'info> {
    pub fn process(
        &mut self,
        max_supply: Option<u32>,
        primary_sale_token_limit: Option<u32>,
        primary_sale_price: Option<u64>,
    ) -> Result<()> {

        let mut token_parameters_data = *self.token_parameters;
        
        if let Some(max_supply) = max_supply {
            token_parameters_data.max_supply = max_supply
        }

        if let Some(primary_sale_token_limit) = primary_sale_token_limit {
            token_parameters_data.primary_sale_token_limit = primary_sale_token_limit
        }

        if let Some(primary_sale_price) = primary_sale_price {
            token_parameters_data.primary_sale_price = primary_sale_price
        }

        *self.token_parameters = token_parameters_data;
        Ok(())
    }
}
