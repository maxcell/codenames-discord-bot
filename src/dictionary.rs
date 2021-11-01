use serenity::{
    builder::CreateButton,
    model::prelude::*,
};

pub struct Word<'a> {
    pub text: &'a str,
    pub custom_id: &'a str
}

impl Word<'static> {
    pub fn build_button(&self) -> CreateButton {
        let mut button: CreateButton = CreateButton::default();
        button.label(self.text);
        button.custom_id(self.custom_id);
        button.style(interactions::message_component::ButtonStyle::Primary);

        button
    }
}
