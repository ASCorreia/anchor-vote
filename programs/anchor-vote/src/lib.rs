use anchor_lang::prelude::*;
use solana_program::hash::hash;

declare_id!("Hs7m6CWPJMBt31EcEiFLs3fRD4m96mC4oWGRnWkG9gpz");

#[program]
pub mod anchor_vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {
        ctx.accounts.vote.score = 0;
        ctx.accounts.vote.bump = ctx.bumps.vote;

        Ok(())
    }

    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.vote.score += 1;

        Ok(())
    }

    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.vote.score -= 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = VoteState::INIT_SPACE,
        seeds = [hash(_url.as_bytes()).to_bytes().as_ref()],
        bump
    )]
    vote: Account<'info, VoteState>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Vote<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [hash(_url.as_bytes()).to_bytes().as_ref()],
        bump = vote.bump
    )]
    vote: Account<'info, VoteState>,
}

#[account]
pub struct VoteState {
    score: i64,
    bump: u8
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8 + 1;
}