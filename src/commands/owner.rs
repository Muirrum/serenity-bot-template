
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::{model::channel::Message, prelude::*};

use crate::prelude::*;

#[command]
#[description = "Restarts the bot"]
#[owners_only]
async fn restart(ctx: &Context, msg: &Message) -> CommandResult {
    match msg
        .channel_id
        .say(&ctx.http, "Restarting bot, and applying new changes")
        .await
    {
        Err(err) => error!("Error sending restart response: {:?}", err),
        Ok(_msg) => (),
    }
    warn!("{} is restarting the bot!", &msg.author.name);
    std::process::exit(0);
}

