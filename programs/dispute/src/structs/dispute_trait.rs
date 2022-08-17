use anchor_lang::prelude::*;


pub trait OrbitDisputableTrait<'a, 'b, T, U>
    where T: Accounts<'a>, U: Accounts<'b>
{
    fn open_dispute(ctx: Context<T>, threshold: u8) -> Result<()>;
    fn close_dispute(ctx: Context<U>) -> Result<()>;
}