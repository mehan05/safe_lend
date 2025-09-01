use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{constants::ANCHOR_DISCRIMINATOR, state::{GlobalState, LoanState, LoanStatus, UserState}};


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct ListLend<'info>{

    #[account(mut)]
    pub admin:AccountInfo<'info>,

    #[account(mut)]
    pub lender: Signer<'info>,


    #[account(
        mut,
        seeds=[b"lender",lender.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub  user_state:Account<'info,UserState>,

    #[account(
        init,
        payer = lender,
        space = ANCHOR_DISCRIMINATOR + LoanState::INIT_SPACE,
        seeds=[b"loan",user_state.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub loan_state:Account<'info,LoanState>,

    #[account(
        init,
        payer = lender,
        associated_token::mint = mint_usdt,
        associated_token::authority = user_state,
        associated_token::token_program = token_program
    )]
    pub lend_vault:InterfaceAccount<'info,TokenAccount>,

    

    

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
    pub lender_ata:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub clock:Sysvar<'info,Clock>
    
}



impl <'info> ListLend<'info>{

    pub fn initialize_lend(&mut self,seed:u64,lend_amount:u64,duration:i64,bumps:ListLendBumps  )->Result<()>{

        let clock  = Clock::get();

        let start_time = clock.unwrap().unix_timestamp;
        let end_time = start_time + duration;

        self.loan_state.set_inner(LoanState{
            lend_amount,
            duration,
            status:LoanStatus::Pending,
            intrest_rate:5,
            seed,
            token:self.mint_usdt.key(),
            start_time,
            end_time,
            lender:self.lender.key(),
            borrower:None,
            bumps:bumps.loan_state,
        });


        Ok(())

    }


    pub fn transfer_funds(&mut self,amount:u64)->Result<()>{


        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked{
            from: self.lender_ata.to_account_info(),
            to: self.lender
        }

    }

}
