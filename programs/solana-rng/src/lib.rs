use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::system_program::Transfer;
use rand_chacha::ChaCha8Rng;
use rand::Rng;
use rand::SeedableRng;

declare_id!("FiQ4npBU1esV384MD7iV4irjVWBwHbQeDrjARoNMgCQ9");

#[program]
pub mod solana_rng {
    use super::*;
    use anchor_lang::{solana_program::native_token::LAMPORTS_PER_SOL, system_program::transfer};

    pub fn initialize_user_state(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();

        let user_state = &mut ctx.accounts.user_state;
        let user = ctx.accounts.user.key();
        user_state.user_key = user;
        // need to change this
        user_state.user_rand_num = None;
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {

        // Generating players random number and saving it to state.
        let slot_hashes_data = ctx.accounts.slot_hashes.try_borrow_data()?;
        let user_state = &mut ctx.accounts.user_state;
        let mut last_blockhash = [0u8; 32];
        last_blockhash.copy_from_slice(&slot_hashes_data[16..48]);

        let mut rng = ChaCha8Rng::from_seed(last_blockhash);
        let random_number: u32 = rng.gen::<u32>() % 100;
        
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

    pub fn settle_bet(ctx: Context<SettleBet>) -> Result<()> {
        // Generating House's random number to compare and settle bet.

        let slot_hashes_data = ctx.accounts.slot_hashes.try_borrow_data()?;
        let user_state = &mut ctx.accounts.user_state;
        let mut last_blockhash = [0u8; 32];
        last_blockhash.copy_from_slice(&slot_hashes_data[16..48]);

        let mut rng = ChaCha8Rng::from_seed(last_blockhash);
        let house_random_number: u32 = rng.gen::<u32>() % 100;
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
    
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = UserState::LEN,
        seeds = [b"user_state", user.key().as_ref()],
        bump,
        // need to correct this constraint
        // constraint = user.key() == user_state.user_key
    )]
    pub user_state: Account<'info, UserState>,
    #[account(mut)]
    pub user: Signer<'info>,
        // Where we store our SOL  
        #[account(
            seeds = [b"vault", state.key().as_ref()],
            bump
        )]
        vault: SystemAccount<'info>,  
        #[account(
            init,
            payer = user,
            space = VaultState::LEN,
            seeds = [b"state", user.key().as_ref()],
            bump
        )]
        state: Account<'info, VaultState>,  
    pub system_program: Program<'info, System>,
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

#[account]
pub struct VaultState {
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub state_bump: u8
}

impl VaultState {
    pub const LEN: usize = 8 + 3 * 1;
}

#[account]
pub struct UserState {
    pub user_key: Pubkey,
    pub user_rand_num: Option<u32>,
}

impl UserState {
    pub const LEN: usize = 8 + 8 + 32;
}
