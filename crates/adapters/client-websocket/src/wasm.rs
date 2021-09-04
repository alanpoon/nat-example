use crate::{command_sender, event_receiver};
use async_trait::async_trait;
use eyre::Result;
use lazy_static::lazy_static;
use protocol::futures::channel::mpsc::channel;
use protocol::futures::future::ready;
use protocol::futures::prelude::*;
use protocol::{Client, ClientName, RawCommand, RawEvent,Event,nats,handle_server_op};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
lazy_static! {
    static ref EVENTS: Mutex<HashMap<ClientName, Vec<Event>>> = Mutex::new(HashMap::default());
    static ref WSEVENTS: Mutex<HashMap<ClientName, Vec<Event>>> = Mutex::new(HashMap::default());
}
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
pub struct WebSocketClient<Tx> {
    client_name: ClientName,
    command_sender: Tx,
    url:String,
    
}

#[async_trait]
impl<Tx> Client for WebSocketClient<Tx>
where
    Tx: Sink<RawCommand, Error = String> + Clone + Send + Sync + Unpin + 'static,
{
    fn sender(&self) -> Box<dyn Sink<RawCommand, Error = String> + Send + Sync + Unpin + 'static> {
        Box::new(self.command_sender.clone())
    }

    fn poll_once(&mut self) -> Option<Vec<Event>> {
        let mut map = EVENTS.lock().unwrap();
        let events = map.get_mut(&self.client_name).unwrap();
        let result = events.clone();
        events.clear();
        events.truncate(10);
        return Some(result);
    }
    fn poll_ws_once(&mut self) -> Option<Vec<Event>> {
      let mut map = WSEVENTS.lock().unwrap();
      let events = map.get_mut(&self.client_name).unwrap();
      let result = events.clone();
      events.clear();
      events.truncate(10);
      return Some(result);
    }
}

pub async fn connect(
    client_name: ClientName,
    url: String,
) -> Result<
    WebSocketClient<impl Sink<RawCommand, Error = String> + Clone + Send + Sync + Unpin + 'static>,
> {
    let mut meta = cross_websocket::connect(url).await?;
    let client_name_c = client_name.clone();
    let mut evt = meta.observe_close().await.unwrap();
    wasm_bindgen_futures::spawn_local(async move{
      console_log!("running");
      if let Some(e)= evt.next().await{
        console_log!("closing {:?}",e);
        
        WSEVENTS
            .lock()
            .unwrap()
            .get_mut(&client_name_c)
            .unwrap()
            .push(Event::WSClose);
            console_log!("push {:?}",EVENTS.lock().unwrap().get_mut(&client_name_c)
            .unwrap().len());
      }
    });
    let (tx, rx)= meta.split();
    let (tx_clone, rx_clone) = channel::<Vec<u8>>(32);
    wasm_bindgen_futures::spawn_local(rx_clone.map(Ok).forward(tx).map(|_| ()));
    
    let event_receiver = event_receiver(rx);
    let result = Ok(WebSocketClient {
        client_name: client_name.clone(),
        command_sender: command_sender(tx_clone.sink_map_err(|err| err.to_string())),
        url:url,
    });
    EVENTS
        .lock()
        .unwrap()
        .insert(client_name.clone(), Vec::new());
    WSEVENTS
        .lock()
        .unwrap()
        .insert(client_name.clone(), Vec::new());
    wasm_bindgen_futures::spawn_local(async {event_receiver.for_each(move |event| {
        // let e = handle_server_op(event).unwrap();
        // if let Some(s_op) = e{
     
          ready(
            EVENTS
                .lock()
                .unwrap()
                .get_mut(&client_name)
                .unwrap()
                //.push(Event::Nats(s_op)),
                .push(Event::Nats(client_name.0.to_string(),event)),
           // ()
          )
        // }else{
        //   ready(())
        // }
      }).await;
      console_log!("end");
    });
    result
}
