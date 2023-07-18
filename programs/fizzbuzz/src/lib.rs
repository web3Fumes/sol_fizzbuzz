use anchor_lang::prelude::*;

declare_id!("4cFTRddTWosQzVL1rya3pUqkAR8khQRiwQ6L4P11Ze5U");

#[program]
pub mod fizzbuzz {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
