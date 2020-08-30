/*
 *   Copyright (c) 2020 Owen Salter <owen@devosmium.xyz>
 *   All rights reserved.
 */

use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::check, Args, CheckResult, CommandOptions},
    model::{
        channel::Message,
        id::{GuildId, RoleId},
    },
};

#[check]
#[name = "Moderator"]
#[display_in_help]
async fn mod_check(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache).await {
        if let Ok(permissions) = member.permissions(&ctx.cache).await {
            return permissions.manage_guild().into();
        }
    }

    false.into()
}

