use anchor_lang::prelude::*;

declare_id!("FbGdm6QCB2X8NvoXbHZmKThkU6EjEJpgQ7CcNx3VTgoA");

#[program]
pub mod gpt_oracle_tuktuk {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
