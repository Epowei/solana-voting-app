use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod states;
pub mod instructions;

pub use constants::ANCHOR_DISCRIMINATOR_SIZE;
pub use errors::ErrorCode::*;
pub use states::*;

#[allow(ambiguous_glob_reexports)]
pub use instructions::*;

declare_id!("9vg2dj78dRHgPopYRsACHCEkjM1SudDT9k1JCyDVxc5k");


#[program]
pub mod votee {
    // use anchor_lang::solana_program::sysvar::instructions as solana_instructions;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn create_poll(ctx: Context<CreatePoll>, description: String, start: u64, end: u64) -> Result<()> {
        instructions::create_poll(ctx, description, start, end)
    }

    pub fn register_candidate(ctx: Context<RegisterCandidate>, poll_id: u64, name: String) -> Result<()> {
        instructions::register_candidate(ctx, poll_id, name)
    }   

    pub fn vote(ctx: Context<Vote>, poll_id: u64, cid: u64) -> Result<()> {
        instructions::vote(ctx, poll_id, cid)
    }
}





