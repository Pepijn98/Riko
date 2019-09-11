mod commands;

use std::{collections::{HashSet}, env, sync::Arc};
use serenity::{
    client::bridge::gateway::{ShardManager, ShardId},
    framework::standard::{
        Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::{group, help, check, command},
    },
    model::{event::ResumedEvent, channel::{Message}, gateway::Ready, id::UserId}
};
use serenity::prelude::*;
use log::{error, info};

use commands::{
    admin::*,
    otaku::*,
    math::*,
    meta::*,
    owner::*,
};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

group!({
    name: "math",
    options: {
        prefix: "math",
    },
    commands: [multiply]
});

group!({
    name: "general",
    options: {},
    commands: [ping, say, latency]
});

group!({
    name: "otaku",
    options: {},
    commands: [anime, manga]
});

group!({
    name: "admin",
    options: {
        checks: [Admin],
    },
    commands: [slow_mode]
});

group!({
    name: "owner",
    options: {
        owners_only: true,
        only_in: "guilds",
    },
    commands: [quit]
});

#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    kankyo::load(false).expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .on_mention(Some(bot_id))
            .prefix("~"))
        .bucket("default", |b| b.delay(5))
        .on_dispatch_error(|ctx, msg, error| {
            if let DispatchError::Ratelimited(seconds) = error {
                let _ = msg.channel_id.say(&ctx.http, &format!("Try this again in {} seconds.", seconds));
            }
        })
        .help(&MY_HELP)
        .group(&ADMIN_GROUP)
        .group(&OTAKU_GROUP)
        .group(&MATH_GROUP)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}

#[check]
#[name = "Owner"]
fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    (msg.author.id == 93973697643155456).into()
}

#[check]
#[name = "Admin"]
#[check_in_help(true)]
#[display_in_help(true)]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {

        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
}

#[command]
fn latency(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

            return Ok(());
        },
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.channel_id.say(&ctx.http, "No shard found");

            return Ok(());
        },
    };

    let _ = msg.channel_id.say(&ctx.http, &format!("The shard latency is {:?}", runner.latency));

    Ok(())
}