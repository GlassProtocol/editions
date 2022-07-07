use crate::{
    error::ProgramError,
    utils::{assert_creator_shares_correct, transfer_sol},
    PrimarySale, PrimarySaleReceipt, TransferSol,
};
use anchor_lang::prelude::*;

impl<'info> PrimarySale<'info> {
    pub fn process(
        &mut self,
        sale_quantity: u32,
        remaining_accounts: &[AccountInfo<'info>],
        mint_authority_seeds: [&[u8]; 2],
        override_primary_sale_price: Option<u64>,
        override_primary_sale_token_limit: Option<u32>,
    ) -> Result<()> {
        msg!("Primary sale");

        let account_limit = override_primary_sale_token_limit
            .unwrap_or(self.token_parameters.primary_sale_token_limit);

        if (self.token_account.amount as u32 + sale_quantity > account_limit)
            || (self.mint.supply as u32 + sale_quantity > self.token_parameters.max_supply)
        {
            return Err(error!(ProgramError::InvalidPrimaryQuantity));
        }

        let metadata = self.metadata.as_ref().data.clone();

        assert_creator_shares_correct(
            metadata.creators.unwrap(),
            metadata.seller_fee_basis_points,
            self.token_parameters.platform_fee_basis_points,
            self.token_parameters.creator_fee_basis_points,
        )?;

        let primary_sale_price = if let Some(sale_quantity) = override_primary_sale_price {
            sale_quantity * sale_quantity as u64
        } else {
            self.token_parameters
                .primary_primary_sale_price(sale_quantity as u64)
        };
        msg!("Sale price {:?}", primary_sale_price);
        let total_fee = self.token_parameters.total_fee(primary_sale_price);
        msg!("Total fee {:?}", total_fee);

        // Distribute fees as specified - convention is platform fee is first creator address in
        // remaining accounts.
        if let Some(creators) = self.metadata.data.creators.clone() {
            let creators_zip = creators.iter().zip(remaining_accounts);
            let creator_mismatch: bool = creators_zip.clone().any(|(c, a)| c.address != a.key());

            if creator_mismatch {
                return Err(error!(ProgramError::InvalidCreators));
            } else {
                let program = self.system_program.to_account_info();
                for (i, (creator, account)) in creators_zip.enumerate() {
                    let mut lamports = total_fee * creator.share as u64 / 100;
                    if i > 0 {
                        lamports += (primary_sale_price - total_fee) * creator.share as u64
                            / (100 - creators[0].share as u64)
                    }
                    msg!("{}", lamports);
                    transfer_sol(
                        CpiContext::new(
                            program.clone(),
                            TransferSol {
                                payer: self.payer.clone(),
                                to: account.to_account_info(),
                                system_program: self.system_program.clone(),
                            },
                        ),
                        lamports,
                    )?;
                }
            }
        }

        let mint_to_ctx = anchor_spl::token::MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };

        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                mint_to_ctx,
                &[&mint_authority_seeds],
            ),
            sale_quantity as u64,
        )?;

        self.token_parameters.number_of_sales += 1;

        // create sale receipt
        let clock = Clock::get()?;
        *self.primary_receipt = PrimarySaleReceipt {
            buyer: self.payer.key(),
            mint: self.mint.key(),
            primary_sale_price: primary_sale_price,
            sale_quantity: sale_quantity as u64,
            created_at: clock.unix_timestamp,
        };
        Ok(())
    }
}
