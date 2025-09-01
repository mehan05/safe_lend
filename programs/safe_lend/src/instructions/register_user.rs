use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{constants::ANCHOR_DISCRIMINATOR, state::{GlobalState, UserState}};


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct RegisterUser<'info>{

    #[account(mut)]
    pub admin:AccountInfo<'info>,

    #[account(mut)]
    pub lender: Signer<'info>,


    #[account(
        init,
        payer = lender,
        space = ANCHOR_DISCRIMINATOR + UserState::INIT_SPACE,
        seeds=[b"lender",lender.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub  user_state:Account<'info,UserState>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_usdt:InterfaceAccount<'info,Mint>,

    #[account(
        init_if_needed,
        payer = lender,
        associated_token::mint = mint_usdt,
        associated_token::authority = lender,
        associated_token::token_program = token_program,
    )]
    pub user_usdt_ata:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
    
    
}


impl <'info> RegisterUser<'info>{
    pub fn register_user(&mut self, bumps:RegisterUserBumps,seed:u64)->Result<()>{
        self.user_state.set_inner(UserState{
            wallet:self.lender.key(),
            active_loans:0,
            completed_loans:0,
            reputation_score:0,
            bumps:bumps.user_state,
            seed
        });

        Ok(())
    }
}