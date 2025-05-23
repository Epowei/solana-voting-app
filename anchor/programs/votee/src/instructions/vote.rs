use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::states::*;
use crate::errors::ErrorCode::*;


pub fn vote(ctx: Context<Vote>, poll_id: u64, cid: u64) -> Result<()> {
    let voter = &mut ctx.accounts.voter;
    let candidate = &mut ctx.accounts.candidate;
    let poll = &mut ctx.accounts.poll;

    if !candidate.has_registered || candidate.poll_id != poll_id {
        return Err(CandidateNotRegistered.into());
    }

    if voter.has_voted {
        return Err(VoterAlreadyVoted.into());
    }

    let current_time = Clock::get()?.unix_timestamp as u64;
    if current_time < poll.start || current_time > poll.end {
        return Err(PollNotActive.into());
    }

    voter.poll_id = poll_id;
    voter.cid = cid;
    voter.has_voted = true;

    candidate.votes += 1;


    Ok(())
}





#[derive(Accounts)]
#[instruction(poll_id: u64, cid: u64)]
pub struct Vote<'info> { 
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), cid.to_le_bytes().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 25,
        seeds = [ b"voter", poll_id.to_le_bytes().as_ref(), user.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>,

    #[account(mut)]
    pub user: Signer<'info>,


    pub system_program: Program<'info, System>,

}
