use tracing::error;

use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher, Event};

use super::normal::Normal;
use wasm_bindgen::prelude::*;
#[derive(Debug, PartialEq, Clone)]
pub struct BeforeLogin {}
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
impl ClientState for BeforeLogin {
    fn handle(&self, _commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
      console_log!("event{:?}",event);
        match event {
            ClientInput::Event(e) => {
                if let Event::Nats(_,s_op)=e{
                  console_log!("{:?}",s_op);
                }
                if let Event::WSClose=e{
                  console_log!("WSClose");
                }
                return Normal {
                    
                }
                .into();
            }
            event => {
                error!("unexpected event: {:?}", event);
            }
        }
        self.clone().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_state::normal::Normal;
    use crate::{ClientContext, UserId};

    #[test]
    fn handles_logged_in_event() {
        let state = BeforeLogin {};
        let mut context = ClientContext {
            ..Default::default()
        };
        let user_id = UserId::generate();
        let event = ClientInput::Event(Event::LoggedIn {
            user_id: user_id.clone(),
        });

        assert_eq!(
            state.handle(&mut context, &event),
            Normal { user_id }.into()
        );
    }
}
