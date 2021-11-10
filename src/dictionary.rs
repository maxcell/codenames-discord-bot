use itertools::Itertools;
use serenity::{
    builder::{CreateActionRow, CreateButton},
    model::prelude::*,
};

pub struct Word {
    pub text: String,
    pub is_touched: bool,
    pub card_type: CardType,
}

impl Word {
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

pub struct Board {
    cards: Vec<Word>,
}

pub enum CardType {
    Red,
    Blue,
    Neutral,
    Assassin,
}

impl Board {
    pub fn create_list() -> Board {
        let mut board = Board { cards: vec![] };
        let list: Vec<Word> = vec![
            Word {
                text: String::from("streak"),
                is_touched: false,
                card_type: CardType::Neutral,
            },
            Word {
                text: String::from("word"),
                is_touched: false,
                card_type: CardType::Blue,
            },
            Word {
                text: String::from("chicken"),
                is_touched: false,
                card_type: CardType::Red,
            },
            Word {
                text: String::from("nuggies"),
                is_touched: false,
                card_type: CardType::Blue,
            },
            Word {
                text: String::from("beef"),
                is_touched: false,
                card_type: CardType::Red,
            },
            Word {
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
