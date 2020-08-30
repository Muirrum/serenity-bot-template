/*
 *   Copyright (c) 2020 Owen Salter <owen@devosmium.xyz>
 *   All rights reserved.
 */

use crate::prelude::*;
use serenity::{
    http::GuildPagination,
    model::{
        guild::Guild,
        guild::PartialGuild,
        id::{ChannelId, GuildId},
        invite::{Invite, InviteGuild},
    },
};

#[command]
#[description = "Pings the bot"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(err) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Err sending message: {}", err);
    };

    Ok(())
}

#[command]
#[description = "Provides helpful information about the bot"]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Serenity Discord Bot Template");
            e.description("A template for building Discord Bots with serenity.rs");
           

            e
        });

        m
    }).await?;

    Ok(())
}

#[command]
#[description = "Displays information about the server"]
#[only_in(guilds)]
async fn serverinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let member_count = guild.member_count;
    let mut guild_owner = guild.owner_id.to_user(&ctx.http).await.unwrap().name;

    let icon_url = match guild.icon_url() {
        Some(url) => url,
        None => String::from("https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fwww.meessendeclercq.be%2Fimages%2Fgallery%2Fartists%2FLDB_Image_Not_Found_web.jpg&f=1&nofb=1")
    };

    guild_owner.push_str("#");
    guild_owner.push_str(
        &guild
            .owner_id
            .to_user(&ctx.http)
            .await
            .unwrap()
            .discriminator
            .to_string(),
    );

    match msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&guild.name);

                e.field("Member Count", member_count.to_string(), true);
                e.field("Server Owner", guild_owner, true);

                e.thumbnail(icon_url);

                e
            });

            m
        })
        .await
    {
        Err(err) => error!("Error sending server count: {:?}", err),
        Ok(_msg) => (),
    }

    Ok(())
}

