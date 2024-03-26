use anchor_lang::prelude::*;

#[account]
pub struct UserState {
    pub user_key: Pubkey,
    pub user_rand_num: Option<u32>,
}

impl UserState {
    pub const LEN: usize = 8 + 8 + 32;
}
