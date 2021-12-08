use itertools::Itertools;
use serenity::{
    builder::{CreateActionRow, CreateButton},
    model::prelude::*,
};
use sqlx::{Executor, PgPool, Row};

use crate::word_bank::{sample_word_bank};

pub struct Card {
    pub text: String,
    pub is_touched: bool,
    pub card_type: CardType,
}

impl Card {
    pub fn build_button(&self) -> CreateButton {
        let mut button: CreateButton = CreateButton::default();
        button.label(self.text.clone());
        button.custom_id(self.text.clone());
        button
    }

    // These represent the cards initial states for the
    // rest of the players (not the spymasters)
    pub fn build_untouched_button(&self) -> CreateButton {
        let mut my_button = self.build_button();
        my_button.style(interactions::message_component::ButtonStyle::Secondary);
        my_button
    }

    // These represent the behind-the-scenes values for
    // the spymasters
    pub fn build_touched_button(&self) -> CreateButton {
        let mut my_button = self.build_button();
        my_button.style(interactions::message_component::ButtonStyle::Secondary);

        let emoji = match self.card_type {
            CardType::Red => String::from("🔴"),
            CardType::Blue => String::from("🔵"),
            CardType::Neutral => String::from("🟤"),
            CardType::Assassin => String::from("☠️"),
        };

        my_button.emoji(serenity::model::channel::ReactionType::Unicode(emoji));
        my_button.disabled(true);
        my_button
    }
}


pub struct Game {
    id: i32
}

pub struct Board {
    cards: Vec<Card>,
}

pub enum CardType {
    Red,
    Blue,
    Neutral,
    Assassin,
}

pub async fn create_game(db_connection: &PgPool, guild_id: u64) {
    sqlx::query!("INSERT INTO server (guild_id) values ($1) ON conflict (guild_id) DO nothing;",
    guild_id as u32)
    .execute(db_connection)
    .await
    .expect("failed to invoke db query");

    let new_game = sqlx::query!("INSERT INTO game (guild_id) values ($1) RETURNING id",
        guild_id as u32
    )
    .fetch_one(db_connection)
    .await
    .expect("Failed to invoke db query");

    let select_words = sample_word_bank(10);

    for word in select_words {
        sqlx::query!("INSERT INTO game_words (word_id,game_id) VALUES ($1, $2)",
            word,
            new_game.id
        ).execute(db_connection)
        .await
        .expect("Failed to invoke db query");
    }
}

impl Board {
    pub fn create_list() -> Board {
        let mut board = Board { cards: vec![] };
        let list: Vec<Card> = vec![
            Card {
                text: String::from("streak"),
                is_touched: false,
                card_type: CardType::Neutral,
            },
            Card {
                text: String::from("word"),
                is_touched: false,
                card_type: CardType::Blue,
            },
            Card {
                text: String::from("chicken"),
                is_touched: false,
                card_type: CardType::Red,
            },
            Card {
                text: String::from("nuggies"),
                is_touched: false,
                card_type: CardType::Blue,
            },
            Card {
                text: String::from("beef"),
                is_touched: false,
                card_type: CardType::Red,
            },
            Card {
                text: String::from("cow"),
                is_touched: false,
                card_type: CardType::Assassin,
            },
        ];

        board.cards = list;
        board
    }

    pub fn build(&self) -> Vec<CreateActionRow> {
        let mut action_rows: Vec<CreateActionRow> = vec![];
        for word_chunks in &self.cards.iter().chunks(5) {
            let mut new_action_row = CreateActionRow::default();

            word_chunks.for_each(|word| {
                new_action_row.add_button(word.build_untouched_button());
            });

            action_rows.push(new_action_row);
        }

        action_rows
    }

    // todo!("make sure to not let this stay here");
    pub fn build_seen(&self) -> Vec<CreateActionRow> {
        let mut action_rows: Vec<CreateActionRow> = vec![];
        for word_chunks in &self.cards.iter().chunks(5) {
            let mut new_action_row = CreateActionRow::default();

            word_chunks.for_each(|word| {
                new_action_row.add_button(word.build_touched_button());
            });

            action_rows.push(new_action_row);
        }

        action_rows
    }
}
