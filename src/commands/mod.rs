pub mod admin;
pub mod otaku;
pub mod math;
pub mod meta;
pub mod owner;

use crate::commands::{
    admin::*,
    otaku::*,
    math::*,
    meta::*,
    owner::*,
};

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    Args, CheckResult, CommandOptions,
    macros::{group, check},
};


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
    commands: [ping, say]
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