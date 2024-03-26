use anchor_lang::prelude::*;
use crate::state::VaultState;
use crate::state::UserState;

pub fn initialize_user_handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
    ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();

    let user_state = &mut ctx.accounts.user_state;
    let user = ctx.accounts.user.key();
    user_state.user_key = user;
    // need to change this
    user_state.user_rand_num = None;
    Ok(())
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