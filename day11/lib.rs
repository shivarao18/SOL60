use anchor_lang::prelude::*;
// prelude contains the Clock struct
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let clock: Clock = Clock::get()?;
    msg!(
        "Block timestamp: {}",
        // Get block.timestamp
        clock.unix_timestamp,
    );
    Ok(())
}