use anchor_lang::prelude::*;
use crate::structs::dispute_struct::OrbitDispute;

#[derive(Accounts)]
pub struct CloseDispute<'info>{
    #[account(mut)]
    pub dispute_acc: Account<'info, OrbitDispute>
}