#[macro_use]
extern crate mongodb;
extern crate serde;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;
extern crate wither;
#[macro_use(Model)]
extern crate wither_derive;

mod commands;
mod keys;
mod models;

use crate::commands::{
    ADMIN_GROUP,
    OTAKU_GROUP,
    MATH_GROUP,
    GENERAL_GROUP,
    OWNER_GROUP
};

use keys::{UptimerKey, Uptimer, ShardManagerContainer};
use log::{error, info};
use std::{collections::{HashSet}, env, sync::Arc};

use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::help,
    },
    model::{event::ResumedEvent, channel::{Message}, guild::Guild, gateway::Ready, id::UserId}
};

use mongodb::{
    ThreadedClient,
    db::DatabaseInner,
};
use wither::prelude::*;

static mut DB: Option<Arc<DatabaseInner>> = None;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    fn guild_create(&self, _: Context, guild: Guild, available: bool) {
        if available {
            let mut guild_model = models::Guild {
                id: None,
                guild_id: guild.id.0
            };

            info!("Joined {} ({})", guild.name, guild_model.guild_id);

            unsafe {
                let db = match &DB {
                    Some(v) => v,
                    None => return error!("DB is None"),
                };
                let _ = guild_model.save(db.clone(), None);
            }
        }
    }
}

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

    let mut client = serenity::Client::new(&token, Handler).expect("Err creating client");

    unsafe {
        DB = Some(mongodb::Client::with_uri("mongodb://localhost:27017/").unwrap().db("riko"));
    }

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<UptimerKey>(Uptimer::new());
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
