use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::TransferChecked, token_interface::{Mint, TokenAccount, TokenInterface,transfer_checked}};

use crate::{constants::ANCHOR_DISCRIMINATOR, state::{GlobalState, LoanState, UserState}};


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct WithDraw<'info>{

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
        mut,
        seeds=[b"loan",user_state.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub loan_state:Account<'info,LoanState>,

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

        #[account(
        mut,
        associated_token::mint = mint_usdt,
        associated_token::authority = user_state,
        associated_token::token_program = token_program
    )]
    pub lend_vault:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
    
    
}

impl<'info> WithDraw<'info> {

    fn transfer_funds(&mut self)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.lend_vault.to_account_info(),
            to: self.lender_ata.to_account_info(),
            authority: self.user_state.to_account_info(),
            mint: self.mint_usdt.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts);
        transfer_checked(ctx,self.loan_state.lend_amount,self.mint_usdt.decimals)?;

        Ok(())
    }
}