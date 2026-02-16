use anchor_lang::prelude::*;

declare_id!("ChorDY11111111111111111111111111111111111");

#[program]
pub mod chordy {
    use super::*;

    pub fn initialize_track(
        ctx: Context<InitializeTrack>,
        title: String,
        royalty_percentage: u8,
    ) -> Result<()> {
        let track = &mut ctx.accounts.track;
        track.creator = *ctx.accounts.creator.key;
        track.title = title;
        track.royalty_percentage = royalty_percentage;
        track.total_earned = 0;

        Ok(())
    }

    pub fn distribute_royalty(
        ctx: Context<DistributeRoyalty>,
        amount: u64,
    ) -> Result<()> {
        let track = &mut ctx.accounts.track;

        **track.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.creator.try_borrow_mut_lamports()? += amount;

        track.total_earned += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTrack<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 64 + 1 + 8)]
    pub track: Account<'info, Track>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeRoyalty<'info> {
    #[account(mut)]
    pub track: Account<'info, Track>,
    #[account(mut, address = track.creator)]
    pub creator: AccountInfo<'info>,
}

#[account]
pub struct Track {
    pub creator: Pubkey,
    pub title: String,
    pub royalty_percentage: u8,
    pub total_earned: u64,
}
