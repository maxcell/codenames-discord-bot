use std::{borrow::{Borrow, BorrowMut}, collections::HashMap, env, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

mod dictionary;

use dictionary::{Board};
use tokio::sync::{RwLockReadGuard, RwLock};
use serenity::{
    async_trait,
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args,
        CommandResult,
        StandardFramework,
    },
    http::Http,
    model::prelude::*,
    prelude::*,
};


struct Game;

impl TypeMapKey for Game {
    type Value = Arc<RwLock<HashMap<u64, Board>>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "codename" => {
                    let options = command
                    .data
                    .options
                    .get(0)
                    .expect("Expected user to provide option")
                    .clone()
                    .resolved
                    .expect("Failed to properly consume the provided option");

                    if let interactions::application_command::ApplicationCommandInteractionDataOptionValue::String(selection) = options {
                    match selection.as_str() {
                        "show" => "Show the board".to_string(),
                        "create" => {
                            let game_lock = {
                                let data_read = ctx.data.read().await;
                                data_read.get::<Game>().expect("Expected Game in TypeMap.").clone()
                            };

                            {
                                let mut game_map = game_lock.write().await;
                                game_map.entry(741467935939231822)
                                .or_insert(Board::create_list());
                            }

                            "New game".to_string()
                        },
                        _ => "Command not implemented".to_string()
                    }
                    } else {
                        "Invalid Option sent to codenames".to_string()
                    }
                },
                _ => "Sorry we don't have a command for that!".to_string()
            };

        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| 
                        message.content(content)
                        .flags(interactions::InteractionApplicationCommandCallbackDataFlags::EPHEMERAL))
            }).await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Slash Command for the guild
        GuildId(741467935939231819)
        .create_application_command(&ctx.http, |command| {
            command.name("codename").description("Commands for Codenames the game")
            .create_option(|option| {
                option.name("command")
                .description("Select the commands you'd like to use")
                .kind(interactions::application_command::ApplicationCommandOptionType::String)
                .add_string_choice(
                    "Start a new game",
                    "create"
                )
                .add_string_choice(
                    "Shows the board",
                    "show"
                )
                .required(true)
            })
        }).await
        .unwrap();
    }
}

#[group("collector")]
#[commands(challenge, show)]
struct Collector;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's id.
    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true).on_mention(Some(bot_id)).prefix("~").delimiters(vec![", ", ","])
        })
        .group(&COLLECTOR_GROUP);

    let mut client = Client::builder(&token)
        .application_id(904223099552149514)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<Game>(Arc::new(RwLock::new(HashMap::default())));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn challenge(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    // Goal: To be able to build the game board
    // In order to do that we need to do <Board Obj>.build()
    // Problem: When trying to get the board state out of the global data (RwLock)
    // it will not allow us to move the data.
    let game_board = {
        let data_read = ctx.data.read().await;
        let game_lock = data_read.get::<Game>()
        .expect("Expected to retrieve game data").clone();

        let game_state = game_lock.read().await;
        // Error here: cannot borrow data in a dereference of `tokio::sync::RwLockReadGuard<'_, HashMap<u64, Board>>` as mutable
        // trait `DerefMut` is required to modify through a dereference, but it is not implemented for `tokio::sync::RwLockReadGuard<'_, HashMap<u64, Board>>`
        game_state.remove(&741467935939231819)
    };
    

    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content( "Let's start a new game");
            m.components(|c| {
                // Ideally we'd want this to workTM
                c.set_action_rows(game_board.unwrap().build())
            });
            m
        })
        .await;

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }

    Ok(())
}

#[command]
async fn show(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    //let _ = msg.reply(ctx, "Let's start a new game").await;
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content( "Let's start a new game");
            m.components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|b| {
                        b.label("beef");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("123123");
                        b
                    });
                    r.create_button(|b| {
                        b.label("steak");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("12123");
                        b
                    });
                    r.create_button(|b| {
                        b.label("ham");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("123");
                        b
                    });
                    r.create_button(|b| {
                        b.label("chicken");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("12");
                        b
                    });
                    r.create_button(|b| {
                        b.label("cow");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("s2");
                        b
                    });
                    r
                });

                c.create_action_row(|r| {
                    r.create_button(|b| {
                        b.label("beef");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("4");
                        b
                    });
                    r.create_button(|b| {
                        b.label("steak");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("5");
                        b
                    });
                    r.create_button(|b| {
                        b.label("ham");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("6");
                        b
                    });
                    r.create_button(|b| {
                        b.label("chicken");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("7");
                        b
                    });
                    r.create_button(|b| {
                        b.label("cow");
                        b.style(interactions::message_component::ButtonStyle::Primary);
                        b.custom_id("8");
                        b
                    });
                    r
                });
                c
            });
            m
        })
        .await;

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }

    Ok(())
}