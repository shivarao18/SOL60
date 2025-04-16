// vectors in rust 

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgvJ3kLrqN1q");

#[program]
pub mod even_filter {
    use super::*;

    pub fn filter_even(_ctx: Context<FilterEven>, input: Vec<u64>) -> Result<()> {
        let mut evens = Vec::new();

        for &val in input.iter() {
            if val % 2 == 0 {
                evens.push(val);
            }
        }

        msg!("Even numbers: {:?}", evens);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FilterEven {}
