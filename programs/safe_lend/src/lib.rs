use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod constants;
declare_id!("6dkxLKKJp4c6ayPoTHtzszMfDndFMFBtxjFa3eUauSTj");

#[program]
pub mod safe_lend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
