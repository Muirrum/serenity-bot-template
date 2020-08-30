/*
 *   Copyright (c) 2020 Owen Salter <owen@devosmium.xyz>
 *   All rights reserved.
 */
use serenity::{
    async_trait,
    framework::standard::{
        help_commands,
        macros::{group, help, hook},
        Args, CommandGroup, CommandResult, DispatchError,
        DispatchError::{
            CheckFailed, CommandDisabled, NotEnoughArguments, OnlyForGuilds, TooManyArguments,
        },
        HelpOptions, StandardFramework,
    },
    http::Http,
    model::{
        channel::{Message, Reaction},
        gateway::{Activity, Ready},
        guild::{Guild, Member},
        id::{ChannelId, GuildId, UserId},
        user::{OnlineStatus, User},
    },
    prelude::*,
    utils::Colour,
};
use std::{collections::HashSet, env};

use log::{debug, error, info};

mod checks;
mod commands;



use commands::{general::*, owner::*};

mod prelude;

// Postgres Connection Pool
#[group]
#[commands(ping, about, serverinfo,)]
struct General;

#[group]
#[commands(restart)]
struct Owner;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let current_user = ready.user;
        info!("Authenticated as {}", current_user.name);
    }

    
}

#[hook]
async fn on_dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        NotEnoughArguments { min, given } => {
            let mut s = format!("Need {} arguments, only got {}.", min, given);
            s.push_str(&" Try using `help <command>` to get usage.");

            match msg.channel_id.say(&ctx, &s).await {
                Err(err) => error!("Error responding to invalid arguments: {:?}", err),
                Ok(_msg) => (),
            }
        }
        TooManyArguments { max, given } => {
            let mut s = format!("Too many arguments. Expected {}, got {}.", max, given);
            s.push_str(" Try using `help <command>` to get usage.");

            match msg.channel_id.say(&ctx, &s).await {
                Err(err) => error!("Error responding to invalid arguments: {:?}", err),
                Ok(_msg) => (),
            }
        }
        CheckFailed(stri, _reason) => {
            info!("{}", stri);
            info!("{} failed to pass check {}", &msg.author.name, stri);

            match msg
                .channel_id
                .say(&ctx, "You do not have permission to use this command!")
                .await
            {
                Err(err) => error!("Error responding to failed check: {:?}", err),
                Ok(_msg) => (),
            }
        }
        OnlyForGuilds => {
            info!(
                "{} tried to use a guild-only command in DMs",
                &msg.author.name
            );
            match msg
                .channel_id
                .say(&ctx, "Please run this command in a Server!")
                .await
            {
                Err(err) => error!("Error sending invalid context msg to {}", &msg.author.name),
                _ => (),
            }
        }
        CommandDisabled(stri) => {
            if let Err(err) = msg
                .channel_id
                .send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Command Error");
                        e.description("That command has been disabled.");
                        e.colour(Colour::RED);

                        e
                    });
                    m
                })
                .await
            {
                error!(
                    "Error sending disabled command message to {}",
                    &msg.author.name
                );
            }
        }
        _ => error!("Unhandled dispatch error."),
    }
}

#[hook]
async fn after(ctx: &Context, msg: &Message, cmd_name: &str, error: CommandResult) {
    if let Err(err) = error {
        error!("Error in {}: {:?}", cmd_name, err);
        if let Err(err) =
            msg.channel_id
                .send_message(&ctx, |m| {
                    m.embed(|e| {
            e.title("Command Error");
            e.description("There was an error running the command.");

            e.colour(Colour::RED);
            e
        });
                    m
                })
                .await
        {
            error!("Error sending error message {:?}", err);
        }
    }
}

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;

    Ok(())
}

#[tokio::main]
async fn main() {
    kankyo::init().expect("Failed to load .env file");
    env_logger::init();

    let token = match env::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(_err) => {
            error!("Could not find discord token in environment");
            String::from("")
        }
    };

    let http = Http::new_with_token(&token);

    debug!("Getting owners");
    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Coudln't get application info: {:?}", why),
    };

    debug!("Initializing Framework");
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .prefix(&env::var("DISCORD_PREFIX").unwrap())
        })
        .after(after)
        .on_dispatch_error(on_dispatch_error)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP)
       .help(&HELP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    debug!("Initializing client");
    let mut disabled_commands: HashSet<String> = HashSet::new();


    info!("Starting client");
    if let Err(err) = client.start().await {
        error!("Client error: {:?}", err);
    }
}
