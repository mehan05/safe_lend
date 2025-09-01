use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{constants::ANCHOR_DISCRIMINATOR, state::GlobalState};

#[derive(Accounts)]
pub struct InitializeLendingPool<'info> {
    

    #[account(mut)]
    pub admin:Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR  + GlobalState::INIT_SPACE,
        seeds=[b"global_state", admin.key().as_ref()],
        bump
    )]
    pub global_state:Account<'info,GlobalState>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_sol:InterfaceAccount<'info,Mint>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_usdt:InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_usdt,
        associated_token::authority = global_state,
        associated_token::token_program = token_program,
    )]
    pub treasure_vault:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl <'info> InitializeLendingPool<'info>{
    pub fn initialize_lending_pool(&mut self, bumps:InitializeLendingPoolBumps)->Result<()>{
        self.global_state.bumps = bumps.global_state;
        Ok(())
    }

}