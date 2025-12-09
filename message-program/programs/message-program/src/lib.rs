use anchor_lang::prelude::*;

declare_id!("Dv6CWyGbEmnj17Zh4x2r2kJ6esDehmVZ3XqdKnsfemEm");

#[program]
pub mod message_program {
    use super::*;

    //  runs only one time to create the account.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;
        message_account.message = "Hello0".to_string();
        message_account.authority = ctx.accounts.authority.key();
        message_account.time_stamp = Clock::get()?.unix_timestamp;

        Ok(())
    }

    // This updates the message later.
    // Only the person who created the account can update it.
    // It also updates the time.
    pub fn update_message(
        ctx: Context<UpdateMessage>,
        message: String
    ) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;

        // Check if the user trying to update is the real owner.
        require!(
            ctx.accounts.authority.key() == message_account.authority,
            ErrorCode::Unauthorized
        );

        message_account.message = message;
        message_account.authority = ctx.accounts.authority.key();
        message_account.time_stamp = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

#[account]
pub struct MessageAccount {
    // The message stored on-chain
    message: String,

    // The person who owns this message account
    authority: Pubkey,

    // When the message was last changed
    time_stamp: i64,
}

impl MessageAccount {
    // Max size allowed for the message
    pub const MAX_MESSAGE_LENGTH: usize = 280;

    // Total space needed for this account
    // 8 = account id
    // 8 = timestamp
    // 4 = string size prefix
    // 280 = message space
    // 32 = pubkey
    pub const SPACE: usize = 8 + 8 + 4 + Self::MAX_MESSAGE_LENGTH + 32;
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Create a fresh account for storing the message
    #[account(
        init,
        payer = authority,
        space = MessageAccount::SPACE
    )]
    pub message_account: Account<'info, MessageAccount>,

    // The user paying for the account
    #[account(mut)]
    pub authority: Signer<'info>,

    // Needed for creating accounts on Solana
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    // The message account that will be updated
    #[account(mut)]
    pub message_account: Account<'info, MessageAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("your not allowed to update this")]
    Unauthorized,
}
