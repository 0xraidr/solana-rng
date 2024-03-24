use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use rand_chacha::ChaCha8Rng;
use rand::Rng;
use rand::SeedableRng;



declare_id!("KkjWcdbexc1SapW67M6dcGhdEJrsrELREt4GhfyPakj");

#[program]
pub mod solana_rng {
    use super::*;

    pub fn initialize_user_state(ctx: Context<Initialize>) -> Result<()> {
        let user_state = &mut ctx.accounts.user_state;
        let user = ctx.accounts.user.key();

        user_state.user_key = user;
        user_state.last_number_generated = 0;
        Ok(())
    }

    pub fn generate_random_number(ctx: Context<GenerateRandom>) -> Result<()> {
        let slot_hashes_data = ctx.accounts.slot_hashes.try_borrow_data()?;
        let user_state = &mut ctx.accounts.user_state;

        let mut last_blockhash = [0u8; 32];
        last_blockhash.copy_from_slice(&slot_hashes_data[16..48]);

        let mut rng = ChaCha8Rng::from_seed(last_blockhash);
        let random_number: u32 = rng.gen::<u32>() % 100;
        
        user_state.last_number_generated = random_number;
        msg!("Random Number: {}", random_number);

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
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GenerateRandom<'info> {
    /// CHECK: just reading data here
    #[account(address = sysvar::slot_hashes::ID)]
    pub slot_hashes: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_state: Account<'info, UserState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[account]
// pub struct VaultState {
//     pub auth_bump: u8,
//     pub vault_bump: u8,
//     pub state_bump: u8
// }

// impl VaultState {
//     pub const LEN: usize = 8 + 3 * 1;
// }

#[account]
pub struct UserState {
    pub user_key: Pubkey,
    pub last_number_generated: u32,
}

impl UserState {
    pub const LEN: usize = 8 + 4 + 32;
}
