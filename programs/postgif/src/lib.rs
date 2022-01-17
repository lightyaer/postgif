use anchor_lang::prelude::*;

declare_id!("BpSyQTheJEzKvc8H5EUUhf2sEg6cJB7YzGy1oj2f1jQr");

#[program]
pub mod postgif {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<Update>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            votes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn up_vote(ctx: Context<Update>, index: i64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let index_usize = index as usize;
        base_account.gif_list[index_usize].votes += 1;
        Ok(())
    }

    pub fn down_vote(ctx: Context<Update>, index: i64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let index_usize = index as usize;

        if base_account.gif_list[index_usize].votes > 0 {
            base_account.gif_list[index_usize].votes -= 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: i64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
