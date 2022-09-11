use anchor_lang::prelude::*;


pub trait OrbitDisputableTrait<'a, 'b, 'c, T, U, V>
    where T: Accounts<'a>, U: Accounts<'b>, V: Accounts<'c>
{
    fn open_dispute(ctx: Context<T>, threshold: u8) -> Result<()>;
    fn close_dispute_sol(ctx: Context<U>, use_discount: bool) -> Result<()>;
    fn close_dispute_spl(ctx: Context<V>, use_discount: bool) -> Result<()>;
}