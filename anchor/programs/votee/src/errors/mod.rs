use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Start date must be before end date")]
    InvalidDates,
    #[msg("Poll not found")]
    PollNotFound,
    #[msg("Candidate already registered")]
    CandidateAlreadyRegistered,
    #[msg("Candidate is not in the poll")]
    CandidateNotRegistered,
    #[msg("Voter has already voted")]
    VoterAlreadyVoted,
    #[msg("Poll is not currently active")]
    PollNotActive,
}