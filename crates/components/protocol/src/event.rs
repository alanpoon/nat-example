use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "c")] // stands for code
pub enum Event {
    #[serde(other)]
    Unknown,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Events(Vec<Event>);

impl Events {
    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.0.iter()
    }

    pub fn push(&mut self, event: Event) {
        self.0.push(event);
    }

    pub fn truncate(&mut self) {
        self.0.clear();
        self.0.truncate(32);
    }
}
