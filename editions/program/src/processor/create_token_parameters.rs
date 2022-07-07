use crate::{
    state::{DataV2, TokenParameters},
    utils::{assert_creator_shares_correct, create_metadata_accounts_v2},
    CreateMetaData, CreateTokenParameters, 
};
use anchor_lang::prelude::*;

impl<'info> CreateTokenParameters<'info> {
    pub fn process(
        &mut self,
        token_parameters_data: TokenParameters,
        metadata_data: DataV2,
        is_mutable: bool,
        mint_authority_seeds: [&[u8]; 2],
    ) -> Result<()> {
        msg!("Create semi-fungible");

        assert_creator_shares_correct(
            metadata_data
                .creators
                .clone()
                .unwrap()
                .iter()
                .map(|c| c.clone().into())
                .collect(),
            metadata_data.seller_fee_basis_points,
            token_parameters_data.platform_fee_basis_points,
            token_parameters_data.creator_fee_basis_points,
        )?;



        create_metadata_accounts_v2(
            CpiContext::new_with_signer(
                self.metadata_program.to_account_info(),
                CreateMetaData::from(self.clone()),
                &[&mint_authority_seeds],
            ),
            false,
            is_mutable,
            metadata_data.into(),
        )?;

        **self.token_parameters = token_parameters_data;
        Ok(())
    }
}
