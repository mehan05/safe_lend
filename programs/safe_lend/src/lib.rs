use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod constants;
pub use instructions::*;
declare_id!("AZW2pKHGYu23m6VTgGgHE9jaRPo1qbJVWp3jXptJKNSg");

#[program]
pub mod safe_lend {

    use super::*;

    pub fn initialize_lend(ctx: Context<InitializeLendingPool>) -> Result<()> {
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
        ctx.accounts.borrow_transfer_funds()
    }

    pub fn repay_funds(ctx: Context<Repay>)->Result<()>{
        ctx.accounts.repay()
    }

    pub fn withdraw_funds(ctx: Context<WithDraw>)->Result<()>{
        ctx.accounts.withdraw_transfer_funds()
    }



    
}
