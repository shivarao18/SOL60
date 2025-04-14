use anchor_lang::prelude::*;

declare_id!("CALCu1ator1111111111111111111111111111111111");

#[program]
pub mod calculator {
    use super::*;

    pub fn add(ctx: Context<CalcCtx>, a: u64, b: u64) -> Result<()> {
        ctx.accounts.result.value = a + b;
        Ok(())
    }

    pub fn sub(ctx: Context<CalcCtx>, a: u64, b: u64) -> Result<()> {
        ctx.accounts.result.value = a.saturating_sub(b);
        Ok(())
    }

    pub fn mul(ctx: Context<CalcCtx>, a: u64, b: u64) -> Result<()> {
        ctx.accounts.result.value = a * b;
        Ok(())
    }

    pub fn div(ctx: Context<CalcCtx>, a: u64, b: u64) -> Result<()> {
        require!(b != 0, CalcError::DivideByZero);
        ctx.accounts.result.value = a / b;
        Ok(())
    }

    pub fn sqrt(ctx: Context<CalcCtx>, x: u64) -> Result<()> {
        ctx.accounts.result.value = (x as f64).sqrt() as u64;
        Ok(())
    }

    pub fn log10(ctx: Context<CalcCtx>, x: u64) -> Result<()> {
        require!(x > 0, CalcError::InvalidLogInput);
        ctx.accounts.result.value = (x as f64).log10() as u64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CalcCtx<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 8, seeds = [b"result", user.key().as_ref()], bump)]
    pub result: Account<'info, CalculatorResult>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CalculatorResult {
    pub value: u64,
}

#[error_code]
pub enum CalcError {
    #[msg("Cannot divide by zero.")]
    DivideByZero,
    #[msg("Cannot compute log10 of zero or negative number.")]
    InvalidLogInput,
}
