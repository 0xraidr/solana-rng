use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;


declare_id!("5ATEphZm1fSphpzBm1rbmKsGyEy9VhXtFJ4n3scV5gwc");

#[program]
pub mod solana_rng {
    use super::*;

    pub fn initialize_user(ctx: Context<Initialize>) -> Result<()> {
        initialize_user_handler(ctx)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {
        buy_ticket_handler(ctx, amount)
    }

    pub fn settle_bet(ctx: Context<SettleBet>) -> Result<()> {
        settle_bet_handler(ctx)
    }

}
