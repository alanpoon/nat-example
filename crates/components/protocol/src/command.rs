use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "c")] // stands for code
pub enum Command {
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod test {
    use super::*;
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Commands(Vec<Command>);
impl Commands {
    pub fn iter(&self) -> impl Iterator<Item = &Command> {
        self.0.iter()
    }

    pub fn push(&mut self, event: Command) {
        self.0.push(event);
    }

    pub fn clear(&mut self) {
        self.0.clear();
        self.0.truncate(32);
    }
}
