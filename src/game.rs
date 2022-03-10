use itertools::Itertools;
use serenity::{
    builder::{CreateActionRow, CreateButton},
    model::prelude::*,
};
use sqlx::{Executor, FromRow, PgPool, Row};

use crate::word_bank::sample_word_bank;

#[derive(Debug, sqlx::FromRow)]
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
    id: i32,
}

#[derive(Debug)]
pub struct Board {
    pub cards: Vec<Card>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "card_type", rename_all = "lowercase")]
pub enum CardType {
    Red,
    Blue,
    Neutral,
    Assassin,
}

pub async fn create_game(db_connection: &PgPool, guild_id: String) {
    sqlx::query!(
        "INSERT INTO new_server (guild_id) values ($1) ON conflict (guild_id) DO nothing;",
        guild_id
    )
    .execute(db_connection)
    .await
    .expect("failed to invoke db query");

    let new_game = sqlx::query!(
        "INSERT INTO new_game (guild_id) values ($1) RETURNING id",
        guild_id
    )
    .fetch_one(db_connection)
    .await
    .expect("Failed to invoke db query");

    let select_words = sample_word_bank(25);

    for (position, word) in select_words.iter().enumerate() {
        sqlx::query!(
            "INSERT INTO game_words (word_id,game_id,card_type) VALUES ($1, $2, $3)",
            word,
            new_game.id,
            match position as u32 {
                0..=7 => CardType::Red,
                8..=16 => CardType::Blue,
                17..=23 => CardType::Neutral,
                24 => CardType::Assassin,
                _ => panic!("Position inserted was incorrect"),
            } as CardType
        )
        .execute(db_connection)
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
