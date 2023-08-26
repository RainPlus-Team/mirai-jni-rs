use crate::model::bot::Bot;

pub trait EventHandler<'a> {
    type ET;

    fn new<F: FnMut(Bot, Self::ET) -> () + 'static>(callback: F) -> Self;

    fn class_name(&self) -> &'static str;

    fn on_event(&mut self, bot: Bot, event_data: Self::ET);
}

pub mod group_message;