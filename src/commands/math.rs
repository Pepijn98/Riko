use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{Args, CommandResult, macros::command},
};

#[command]
#[bucket = "default"]
pub fn multiply(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let product = one * two;

    let _ = msg.channel_id.say(&ctx.http, product);

    Ok(())
}