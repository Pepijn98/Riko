use crate::keys::{UptimerKey, ShardManagerContainer};

use log::{error};

use chrono::DateTime;
use chrono::Utc;

use serenity::{
    prelude::*,
    model::prelude::*,
    client::bridge::gateway::ShardId,
    utils::{ContentSafeOptions, content_safe, Colour},
    framework::standard::{CommandResult, CommandError, macros::command, Args},
};

struct Timer {
    start: DateTime<Utc>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start: Utc::now(),
        }
    }

    pub fn elapsed_ms(&self) -> i64 {
        Utc::now()
            .signed_duration_since(self.start)
            .num_milliseconds()
    }
}

#[command]
#[bucket = "default"]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let timer = Timer::new();

    let mut sent_msg = match msg.channel_id.say(&ctx.http, "Ping!") {
        Ok(m) => m,
        Err(_) => return Ok(()),
    };

    let msg_ms = timer.elapsed_ms();

    // shard latency
    let runner_latency = {
        let data = ctx.data.read();
        let shard_manager = match data.get::<ShardManagerContainer>() {
            Some(v) => v,
            None => return Err(CommandError::from("There was a problem getting the shard manager")),
        };

        let manager = shard_manager.lock();
        let runners = manager.runners.lock();

        let runner = match runners.get(&ShardId(ctx.shard_id)) {
            Some(runner) => runner,
            None => return Err(CommandError::from("No shard found")),
        };

        runner.latency
    };

    let runner_latency_ms = runner_latency.map(|x|
        format!("{:.3}", x.as_secs() as f64 / 1000.0 + f64::from(x.subsec_nanos()) * 1e-6)
    );

    let _ = sent_msg.edit(&ctx, |m| m.content(
            &format!(
                "Discord Rest API (message send): `{} ms`\n\
                Discord Shard latency (heartbeat ACK): `{} ms`\n",
                msg_ms,
                runner_latency_ms.unwrap_or("N/A".into())
            )
        )
    );

    Ok(())
}

#[command]
#[bucket = "default"]
fn uptime(ctx: &mut Context, msg: &Message) -> CommandResult {
    let uptime_string = {
        let data = ctx.data.read();
        let uptimer = match data.get::<UptimerKey>() {
            Some(v) => v,
            None => {
                let _ = msg.channel_id.say(&ctx.http, "There was a problem getting the uptime");

                return Ok(())
            }
        };

        uptimer.uptime_string()
    };

    let _ = match msg.channel_id.send_message(&ctx.http, |cm| cm
        .embed(|ce| ce
            .title("Uptime")
            .description(&uptime_string)
            .color(Colour::from(000000)))
    ){
        Ok(msg) => msg,
        Err(why) => {
            let _ = msg.channel_id.say(&ctx.http, format!("Error sending embed:\n{:?}", why));
			return Ok(());
        }
    };

    Ok(())
}

#[command]
#[bucket = "default"]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
       ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings);

    if let Err(why) = msg.channel_id.say(&ctx.http, &content) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}