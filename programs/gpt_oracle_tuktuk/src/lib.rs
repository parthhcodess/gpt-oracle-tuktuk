#![allow(deprecated, unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod constants;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("FbGdm6QCB2X8NvoXbHZmKThkU6EjEJpgQ7CcNx3VTgoA");

#[program]
pub mod gpt_oracle_tuktuk {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn callback_from_llm(ctx: Context<Callback>, response: String) -> Result<()> {
        ctx.accounts.callback_from_llm(response)?;
        Ok(())
    }

    pub fn interact_with_llm(ctx: Context<Interact>) -> Result<()> {
        ctx.accounts.interact_with_llm()?;
        Ok(())
    }

    pub fn schedule(ctx: Context<Schedule>, task_id: u16) -> Result<()> {
        ctx.accounts.schedule(task_id, &ctx.bumps)?;
        Ok(())
    }
}