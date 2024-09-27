use anchor_lang::prelude::*;
use anchor_spl::{metadata::{
    mpl_token_metadata::instructions::{ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts},
    MasterEditionAccount,
    Metadata,
    MetadataAccount
}, token::{revoke, Mint, Revoke, Token, TokenAccount}};

use crate::{state::{StakeAccount, StakeConfig, UserAccount}, error::ErrorCode};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    user: Signer<'info>,
    mint: Account<'info, Mint>,
    collection: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    edition: Account<'info, MasterEditionAccount>,
    config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    user_account: Account<'info, UserAccount>,
    #[account(
        init,
        payer = user,
        space = StakeAccount::INIT_SPACE,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    stake_account: Account<'info, StakeAccount>,
    metadata_program: Program<'info, Metadata>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let days_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.last_update) / (24 * 60 * 60)) as u32;
        require!(days_elapsed > self.config.freeze_period, ErrorCode::UnstakeFreezeDurationInvalid);

        self.user_account.points += days_elapsed * self.config.points_per_stake as u32;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let mint = &self.mint.to_account_info();
        let edition = &self.edition.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        ThawDelegatedAccountCpi::new(
            metadata_program,
            ThawDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            }
        ).invoke()?;

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            source: self.mint_ata.to_account_info(),
            authority: self.stake_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        revoke(cpi_ctx)?;
        
        Ok(())
    }
}