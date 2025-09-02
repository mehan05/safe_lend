use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{constants::ANCHOR_DISCRIMINATOR, state::{GlobalState, LoanState, LoanStatus, UserState}};


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Repay<'info>{

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

    #[account(
        init_if_needed,
        payer = lender,
        associated_token::mint = mint_usdt,
        associated_token::authority = lender,
        associated_token::token_program = token_program,
    )]
    pub lender_ata:InterfaceAccount<'info,TokenAccount>,

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
       mut,
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

impl <'info> Repay<'info>{
    pub fn repay(&mut self)->Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts_borrower_to_vault = TransferChecked{
            from: self.borrower_ata.to_account_info(),
            to: self.borrower_vault.to_account_info(),
            authority: self.borrower.to_account_info(),
            mint: self.mint_sol.to_account_info(),
        };

        let repay_amount_with_intrest = self.loan_state.lend_amount.checked_add(self.loan_state.lend_amount.checked_add(self.loan_state.intrest_rate.checked_div(100).unwrap()).unwrap()).unwrap();

        let ctx = CpiContext::new(cpi_program.clone(),cpi_accounts_borrower_to_vault);
        transfer_checked(ctx,repay_amount_with_intrest,self.mint_usdt.decimals)?;
        
        let cpi_accounts_vault_to_borrower = TransferChecked{
            from: self.borrower_vault.to_account_info(),
            to: self.borrower_ata.to_account_info(),
            authority: self.user_state.to_account_info(),
            mint: self.mint_sol.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts_vault_to_borrower);
        transfer_checked(ctx,self.loan_state.collateral_amount,self.mint_usdt.decimals)?;

        Ok(())

    }
}