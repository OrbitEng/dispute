use anchor_lang::prelude::*;

#[account]
pub struct OrbitDispute{
    pub transaction: Pubkey,
    pub favor: Pubkey,
}
