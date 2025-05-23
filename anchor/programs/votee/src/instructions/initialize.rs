use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::states::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.count = 0;
    let registrations = &mut ctx.accounts.registrations;
    registrations.count = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8,
        seeds = [b"counter".as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8,
        seeds = [b"registrations".as_ref()],
        bump
    )]
    pub registrations: Account<'info, Registrations>,

    pub system_program: Program<'info, System>,

}