use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod constants;

declare_id!("6dkxLKKJp4c6ayPoTHtzszMfDndFMFBtxjFa3eUauSTj");

#[program]
pub mod safe_lend {
    use super::*;

    pub fn initialize(ctx: Context<InitializeLend>) -> Result<()> {
        ctx.accounts.initialize_lending_pool(ctx.bumps)?;
        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>,seed:u64) -> Result<()> {
        ctx.accounts.register_user(ctx.bumps,seed)
    }

    pub fn list_lend(ctx: Context<ListLend>,seed:u64,lend_amount:u64,duration:i64) -> Result<()> {
        ctx.accounts.initialize_lending(seed,lend_amount,duration,ctx.bumps)
    }

    pub fn  borrow(ctx: Context<Borrow>)->Result<()>{
        ctx.accounts.borrow()
    }

    pub fn repay(ctx: Context<Repay>)->Result<()>{
        ctx.accounts.repay()
    }

    pub fn withdraw(ctx: Context<Withdraw>)->Result<()>{
        ctx.accounts.withdraw()
    }



    
}

#[derive(Accounts)]
pub struct Initialize {}
