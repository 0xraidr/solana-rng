use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::system_program::Transfer;
use rand_chacha::ChaCha8Rng;
use rand::Rng;
use rand::SeedableRng;
use anchor_lang::system_program::transfer;

use crate::state::VaultState;
use crate::state::UserState;


pub fn settle_bet_handler(ctx: Context<SettleBet>) -> Result<()> {
    // Generating House's random number to compare and settle bet.

    let slot_hashes_data = ctx.accounts.slot_hashes.try_borrow_data()?;
    let user_state = &mut ctx.accounts.user_state;
    let mut last_blockhash = [0u8; 32];
    last_blockhash.copy_from_slice(&slot_hashes_data[16..48]);

    let mut rng = ChaCha8Rng::from_seed(last_blockhash);
    let house_random_number: u32 = rng.gen::<u32>() % 3;
    msg!("House Random Number: {}", house_random_number);

    // Need to come back to this part, but ive got to link the jackpot to
    // this ix so that we can calculate how much to pay out.
    if Some(house_random_number) == user_state.user_rand_num {
        let accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.vault_bump][..]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accounts,
            signer_seeds
        );
        msg!("Winner! Winner! Chicken! Dinner!");

        transfer(cpi, 1)?
    }
    msg!("Try Again Bozo!");

    Ok(())
}

#[derive(Accounts)]
pub struct SettleBet<'info> {
    #[account(mut)]
    user: Signer<'info>,
    // Where we store our SOL  
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump
    )]
    vault: SystemAccount<'info>,  
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump
         =  state.state_bump
    )]
    state: Account<'info, VaultState>, 

     /// CHECK: just reading data here
     #[account(address = sysvar::slot_hashes::ID)]
    pub slot_hashes: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    system_program: Program<'info, System>
}