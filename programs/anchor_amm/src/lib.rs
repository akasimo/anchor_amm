use anchor_lang::prelude::*;

declare_id!("2oAPYdwKv92TZr6YELKy4TLXCQxSz16cLzSQ5w7tvFJs");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
