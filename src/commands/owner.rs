use crate::ShardManagerContainer;

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{CommandResult, macros::command},
};

#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        manager.lock().shutdown_all();
    } else {
        let _ = msg.channel_id.say(&ctx.http, "There was a problem getting the shard manager");

        return Ok(());
    }

    let _ = msg.channel_id.say(&ctx.http, "Shutting down!");

    Ok(())
}