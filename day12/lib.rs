use anchor_lang::prelude::*;
declare_id!("...");

pub const Discriminator_size : usize = 8;

pub mod favourites{
    use super::*;
    pub fn set_favourite(context : Context<SetFavourites>, number : u64, colour : String, hobbies : Vec<String>) -> Result<()>{
        let user = context.accounts.user;
        context.accounts.favourites.set_inner(Favourites{
            number,
            colour,
            hobbies,
        })?;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
public struct Favourites{
    pub number: u64,
    #[max_len(50)]
    pub colour: String,
    #[max_len(5,50)]
    public hobbies : Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    #[account(init_if_needed, payer = user, space = Discriminator_size + Favourites::InitSpace, 
        seeds = [b"favourites", user.key().as_ref()], bump)]
    pub favourites : Account<'info, Favourites>,
    pub system_program : Program<'info, System>,
}

