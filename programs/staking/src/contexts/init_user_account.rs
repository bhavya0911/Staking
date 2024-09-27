use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
        space = UserAccount::INIT_SPACE,
    )]
    user_account: Account<'info, UserAccount>,
    system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init_user(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });
        
        Ok(())
    }
}