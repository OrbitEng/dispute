use orbit_physical_market::structs::physical_transaction::PhysicalTransaction;

#[derive(Accounts)]
pub struct OpenPhysicalDispute<'info>{
    #[account(
        init,
        space = 100,
        payer = payer
    )]
    pub new_dispute: Account<'info, OrbitDispute>,

    pub transaction: Account<'info, PhysicalTransaction>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        seeds = [b"dispute_auth"],
        bump
    )]
    pub dispute_auth: AccountInfo<'info>,
}

pub fn open_physical_dispute(ctx: Context<OpenPhysicalDispute>) -> Result<()>{

    Ok(())
}