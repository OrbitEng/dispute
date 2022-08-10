use anchor_lang::prelude::*;


pub trait OrbitDisputableTrait<'a, 'b, T, U>{
    fn freeze(ctx: Context<T>) -> Result<()>;
    fn unfreeze(ctx: Context<U>) -> Result<()>;
}