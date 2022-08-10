use anchor_lang::prelude::*;


pub trait OrbitDisputableTrait<'a, 'b, T, U>
    where T: Accounts<'a>, U: Accounts<'b>
{
    fn freeze(ctx: Context<T>) -> Result<()>;
    fn unfreeze(ctx: Context<U>) -> Result<()>;
}