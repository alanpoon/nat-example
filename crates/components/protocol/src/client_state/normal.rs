use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct Normal {
  //user_id
}

impl ClientState for Normal {
    fn handle(&self, _commands: &mut ClientContext, _event: &ClientInput) -> ClientStateDispatcher {
        self.clone().into()
    }
}
