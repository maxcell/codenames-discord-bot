use std::{env};

mod dictionary;

use dictionary::Word;

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

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "codename" => "Hey, I'm alive!".to_string(),
                _ => "Sorry not implemented".to_string()
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

        // Test Guild ID - 741467935939231819

        // Slash Command for the guild
        let guild_command = GuildId(741467935939231819)
        .create_application_command(&ctx.http, |command| {
            command.name("codename").description("A way to show the board")
        })
        .await;
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

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn challenge(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let text_word = Word {
        text: "word",
        custom_id: "123123"
    };
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content( "Let's start a new game");
            m.components(|c| {
                c.create_action_row(|r| {
                    r.add_button(text_word.build_button());
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

                // c.create_action_row(|r| {
                //     r.create_button(|b| {
                //         b.label("beef");
                //         b.style(interactions::message_component::ButtonStyle::Primary);
                //         b.custom_id("4");
                //         b
                //     });
                //     r.create_button(|b| {
                //         b.label("steak");
                //         b.style(interactions::message_component::ButtonStyle::Primary);
                //         b.custom_id("5");
                //         b
                //     });
                //     r.create_button(|b| {
                //         b.label("ham");
                //         b.style(interactions::message_component::ButtonStyle::Primary);
                //         b.custom_id("6");
                //         b
                //     });
                //     r.create_button(|b| {
                //         b.label("chicken");
                //         b.style(interactions::message_component::ButtonStyle::Primary);
                //         b.custom_id("7");
                //         b
                //     });
                //     r.create_button(|b| {
                //         b.label("cow");
                //         b.style(interactions::message_component::ButtonStyle::Primary);
                //         b.custom_id("8");
                //         b
                //     });
                //     r
                // });
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