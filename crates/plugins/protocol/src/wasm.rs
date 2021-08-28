use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use client_websocket::connect;
use futures::future::ready;
use futures::prelude::*;
use futures::future::{join_all, ok, err};
use lazy_static::lazy_static;
use protocol::{BoxClient, ClientName,nats};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::error;
use wasm_bindgen_futures::spawn_local;
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
const DEFAULT_CLIENT: ClientName =
    ClientName(Cow::Borrowed("desk-plugin-protocol: default client"));

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<ClientName, BoxClient>> = Mutex::new(HashMap::new());
}

pub fn connect_websocket() {
    let servers=vec![String::from("ws://127.0.0.1:5000/ws")];
    
    let future_arr= servers.iter().map(|s|{
      connect(DEFAULT_CLIENT,s.to_string()).then(|c|{
        ready(c
          .map(|client| Box::new(client))
          // .map(|client| {
          //   bc.clients.push(client);
          // })
        )
      })
    });
    let join_ = join_all(future_arr).then(|l|{
      let mut bc = BoxClient::default();
      for n in l{
        if let Ok(z) = n{
          bc.clients.push(z);
        }
      }
      CLIENTS.lock().unwrap().insert(DEFAULT_CLIENT, bc);
      ready(())
      
      });
    spawn_local(join_);
    
}

pub fn set_client(mut client_res: ResMut<Option<BoxClient>>) {
    let mut map = CLIENTS.lock().unwrap();
    if let Some(client) = map.remove(&DEFAULT_CLIENT) {
        *client_res = Some(client);
    }
}

pub fn block_on<T>(future: impl Future<Output = T> + 'static) {
    wasm_bindgen_futures::spawn_local(async { future.map(|_| ()).await });
}
pub fn get_random_int(min:i32,max:i32)->usize{
  ((js_sys::Math::floor(js_sys::Math::random()) as i32) *(max-min)+min) as usize
}
// pub fn dial_loop(mut client_res: ResMut<Option<BoxClient>>){
//   // spawn_local(
//   //   async{
//       console_log!("before delay{:?}",js_sys::Date::new_0());
//       delay(40002).await;
//       console_log!("after delay{:?}",js_sys::Date::new_0());
//       let mut map = CLIENTS.lock().unwrap();
//       console_log!("heloo");
//   //   }
//   // )
// }
pub async fn delay(timeout_ms: i32)->(){
  let p = js_sys::Promise::new(&mut |resolve, _| {
    let closure = Closure::wrap(Box::new(move || {
      //resolve(&42.into())
      resolve.call0(&JsValue::NULL);
    })as Box<dyn FnMut()>);
    
    set_timeout(&closure,timeout_ms);
    closure.forget();
    }
    
  );
 wasm_bindgen_futures::JsFuture::from(p).into_future().await;
 ()
}
fn set_timeout(f: &Closure<dyn FnMut()>,timeout_ms: i32) {
  window()
      .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(),timeout_ms)
      .expect("should register `requestAnimationFrame` OK");
}
fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}