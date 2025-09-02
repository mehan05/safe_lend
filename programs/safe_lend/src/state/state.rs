use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize,Clone,)]
pub enum LoanStatus{
    Active,
    Completed,
    Pending
}

impl anchor_lang::Space for LoanStatus{
    const INIT_SPACE:usize = 1;
}

#[account]
#[derive(InitSpace)]
pub struct GlobalState{
    pub total_loans:u64,
    pub treasure_fees:u64,
    pub authority : Pubkey,
    pub platform_fee:u64,   
    pub bumps:u8
}


#[account]
#[derive(InitSpace)]
pub struct UserState{
    pub wallet:Pubkey,
    pub active_loans:u64,
    pub completed_loans:u64,
    pub reputation_score:u64,
    pub bumps:u8,
    pub seed:u64
}

#[account]
#[derive(InitSpace)]
pub struct LoanState{
    pub lend_amount:u64,
    pub intrest_rate:u64,
    pub start_time:Option<i64>,
    pub end_time:Option<i64>,
    pub duration:i64,
    pub status:LoanStatus,
    pub token:Pubkey,
    pub lender:Pubkey,
    pub borrower:Option<Pubkey>,
    pub bumps:u8,
    pub seed:u64
}