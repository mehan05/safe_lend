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

    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        init_if_needed,
        payer = lender,
        associated_token::mint = mint_sol,
        associated_token::authority = lender,
        associated_token::token_program = token_program,
    )]
    pub borrower_ata:InterfaceAccount<'info,TokenAccount>,

    pub mint_sol:InterfaceAccount<'info,Mint>,
    pub mint_usdt:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        seeds=[b"global_state", admin.key().as_ref()],
        bump
    )]
    pub global_state:Account<'info,GlobalState>,

    #[account(
        mut,
        seeds=[b"lender",lender.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub  user_state:Account<'info,UserState>,

    #[account(
        mut,
        seeds=[b"loan",user_state.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub loan_state:Account<'info,LoanState>,

    #[account(
        mut,
        associated_token::mint = mint_usdt,
        associated_token::authority = user_state,
        associated_token::token_program = token_program
    )]
    pub lend_vault:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer = borrower,
        associated_token::mint = mint_sol,
        associated_token::authority = user_state,
        associated_token::token_program = token_program
    )]
    pub borrower_vault:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub clock:Sysvar<'info,Clock>
    
}

impl<'info> ListLend<'info>{

       pub fn transfer_funds(&mut self,amount:u64)->Result<()>{


        let cpi_program = self.token_program.to_account_info();


        
        let cpi_accounts = TransferChecked{
            from: self.borrower_ata.to_account_info(),
            to: self.borrower_vault.to_account_info(),
            authority: self.lender.to_account_info(),
            mint: self.mint_usdt.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program.clone(),cpi_accounts);
        transfer_checked(ctx,amount,self.mint_usdt.decimals)?;


        let cpi_accounts_lend_amount = TransferChecked{
            from: self.lend_vault.to_account_info(),
            to: self.borrower_ata.to_account_info(),
            authority: self.lender.to_account_info(),
            mint: self.mint_usdt.to_account_info(),
        
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts_lend_amount);
        transfer_checked(ctx,amount,self.mint_usdt.decimals)?;

             let clock: std::result::Result<Clock, ProgramError>  = Clock::get();

        let start_time = clock.unwrap().unix_timestamp;
        let end_time = start_time + self.loan_state.duration;


        self.user_state.active_loans.checked_add(1);
        self.loan_state.status = LoanStatus::Active;
        self.loan_state.borrower = Some(self.borrower.key());
        self.loan_state.start_time = Some(start_time);
        self.loan_state.end_time = Some(end_time);



        Ok(())

    }


}

