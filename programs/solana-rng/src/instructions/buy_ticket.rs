use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::system_program::Transfer;
use rand_chacha::ChaCha8Rng;
use rand::Rng;
use rand::SeedableRng;
use crate::state::VaultState;
use crate::state::UserState;
use anchor_lang::system_program::transfer;



pub fn buy_ticket_handler(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {

    // Generating players random number and saving it to state.
    let slot_hashes_data = ctx.accounts.slot_hashes.try_borrow_data()?;
    let user_state = &mut ctx.accounts.user_state;
    let mut last_blockhash = [0u8; 32];
    last_blockhash.copy_from_slice(&slot_hashes_data[16..48]);

    let mut rng = ChaCha8Rng::from_seed(last_blockhash);
    let random_number: u32 = rng.gen::<u32>() % 3;
    
    user_state.user_rand_num = Some(random_number);
    msg!("Random Number: {}", random_number);

    // Depositing bet fund into vault(escrow) until next ix.
    let accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        accounts,
    );
    transfer(cpi, amount)
}


#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    user: Signer<'info>,
    // Where we store our SOL  
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump 
        = state.vault_bump
    )]
    vault: SystemAccount<'info>,  
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump =  state.state_bump
    )]
    state: Account<'info, VaultState>, 

     /// CHECK: just reading data here
     #[account(address = sysvar::slot_hashes::ID)]
    pub slot_hashes: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    system_program: Program<'info, System>
}