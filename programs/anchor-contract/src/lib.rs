use anchor_lang::prelude::*;


declare_id!("CKyE2drXuaYcbBF8japHFSQTgEShYfGBtZKjJuN1nMT3");
#[error_code]
pub enum MyError {
    #[msg("自定义错误")]
    DataTooLarge
}
#[program]
mod anchor_contract {
    use super::*;

    pub fn create(ctx: Context<Create>, data: String) -> Result<()> {
        let pad_account = &mut ctx.accounts.pad_account;
        pad_account.data = data;
        pad_account.receiver = ctx.accounts.payer.key.clone();
        let x = ctx.bumps.get("reward_mint");
        Ok(())
    }

    pub fn modification(ctx: Context<Modification>, data: String) -> Result<()> {
        let pad_account = &mut ctx.accounts.pad_account;
        pad_account.data = data;
        Ok(())
    }

    pub fn delete(_ctx: Context<Delete>) -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct DataAccount {
    data: String,
    receiver: Pubkey,
}


#[derive(Accounts)]
#[instruction(data: String)]
pub struct Create<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [payer.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + 4 + data.len()
    )]
    pub pad_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(data: String)]
pub struct Modification<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [payer.key().as_ref()],
        bump,
        realloc = 8 + 4 + data.len(),
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub pad_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(mut, close = receiver, has_one = receiver)]
    pub pad_account: Account<'info, DataAccount>,

}
