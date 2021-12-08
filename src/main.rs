use std::{collections::HashMap, convert::TryFrom, env, sync::{
        Arc,
    }};

mod game;

use game::{Board,create_game};
use serenity::{async_trait, framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    }, 
    model::interactions::application_command::ApplicationCommandInteractionDataOptionValue,
    http::Http, model::prelude::*, prelude::*};
use tokio::sync::{RwLock};

mod word_bank;


struct Game;

impl TypeMapKey for Game {
    type Value = Arc<RwLock<HashMap<u64, Board>>>;
}

struct Db;
impl TypeMapKey for Db {
    type Value = Arc<sqlx::PgPool>;
}

struct CodenameCommand { 
    data: interactions::application_command::ApplicationCommandInteraction,
    option: interactions::application_command::ApplicationCommandInteractionDataOptionValue
}

enum SubCommand {
    Show
}

impl CodenameCommand {
    fn content(&self) -> String {
        if let ApplicationCommandInteractionDataOptionValue::String(selection) = self.option.clone() {
            match selection.as_str() {
                "show" => "Here's the secret board. Don't speak to anyone about it!".to_string(),
                "create" => {
                    "New Game Created!".to_string()
                },
                _ => "Command not implemented".to_string()
            }
        } else {
            "Invalid Option sent to the command".to_string()
        }
    }
}

impl TryFrom<interactions::application_command::ApplicationCommandInteraction> for CodenameCommand {
    type Error = ();

    fn try_from(value: interactions::application_command::ApplicationCommandInteraction) -> Result<Self, Self::Error> {
        if value.data.name.as_str() == "codename"  {
            
            let option = value
            .data
            .options
            .get(0)
            .expect("Expect receive option")
            .clone()
            .resolved
            .expect("Failed to use provided option");

           return Ok(CodenameCommand{ data: value, option });
            
        }
        
        return Err(());
    }
}



struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let game_board = {
            let game_arc_lock = ctx
                .data
                .read()
                .await
                .get::<Game>()
                .expect("Expected to retrieve game data")
                .clone();

            game_arc_lock
        };

        let database_connection = {
            let arc = ctx
            .data
            .read()
            .await;
    
            arc.get::<Db>()
            .expect("Failed to get connection to database")
            .clone()
        };

        if let Interaction::ApplicationCommand(command) = interaction {
            let codename_struct = CodenameCommand::try_from(command.clone()).unwrap();
            
            let board = game_board.read().await;

            if let Err(why) = command.clone() 
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        let message = message.content(codename_struct.content());
                        if let ApplicationCommandInteractionDataOptionValue::String(selection) = codename_struct.option {
                            match selection.as_str() {
                                "show" => {
                                    message.components(|c| {                                        
                                        c.set_action_rows(
                                            board.get(&741467935939231822).unwrap().build_seen()
                                        )
                                    })
                                    .flags(interactions::InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                                },
                                "create" => {
                                    tokio::spawn(async move {
                                        let guild_id = command.guild_id.unwrap();
                                        create_game(&database_connection, guild_id.as_u64().clone()).await;
                                    });
                                    message
                                },
                               _ => {
                                   message
                               }
                            }
                        } else {
                            message.content("You fucked up".to_string())
                        }
                        
            })
        }).await
        {
                println!("Cannot respond to slash command: {}", why);
        }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let game_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<Game>()
                .expect("Expected Game in TypeMap.")
                .clone()
        };

         // Initializing game board data
         // We may want to replace this with a request on retrieving all
         // games currently within the server
        {
            let mut game_map = game_lock.write().await;
            game_map
                .entry(741467935939231822)
                .or_insert(Board::create_list());
        }

        println!("{} is connected!", ready.user.name);

        // Slash Command for the guild
        GuildId(741467935939231819)
            .create_application_command(&ctx.http, |command| {
                command
                    .name("codename")
                    .description("Commands for Codenames the game")
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
            })
            .await
            .unwrap();
    }
}

#[group("collector")]
#[commands(show)]
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
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("~")
                .delimiters(vec![", ", ","])
        })
        .group(&COLLECTOR_GROUP);
    
    let database_url = std::env::var("DATABASE_URL").expect("Make sure to add the DATABASE_URL to the environment variables");

    let database = sqlx::postgres::PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await.expect("Test if we made teh connection");

    // ? Why did we need to use expect here as opposed to question mark?
    // Is it due to the impl future

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

    {
        let mut data = client.data.write().await;
        data.insert::<Db>(Arc::new(database));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn show(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let game_board = {
        let game_arc_lock = ctx
            .data
            .read()
            .await
            .get::<Game>()
            .expect("Expected to retrieve game data")
            .clone();

        game_arc_lock
    };

    let board = game_board.read().await;
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Let's start a new game");
            m.components(|c| {
                // Ideally we'd want this to workTM
                c.set_action_rows(board.get(&741467935939231822).unwrap().build())
            });
            m
        })
        .await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}