use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::utils::{ContentSafeOptions, content_safe};
use serenity::framework::standard::{CommandResult, macros::command, Args};

#[command]
#[bucket = "default"]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}

#[command]
#[bucket = "default"]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
       ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings);

    if let Err(why) = msg.channel_id.say(&ctx.http, &content) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}