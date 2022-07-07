use crate::{state::PlatformSettings, CreatePlatformSettings};
use anchor_lang::prelude::*;

impl<'info> CreatePlatformSettings<'info> {
    pub fn process(&mut self, data: PlatformSettings) -> Result<()> {
        msg!("Create platform settings");
        *self.platform_settings = data;
        Ok(())
    }
}
