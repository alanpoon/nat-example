
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
use native::*;

use bevy::prelude::*;
use core::ProtocolSystem;
use futures::prelude::*;
use protocol::{BoxClient, ClientContext, ClientInput, ClientState, ClientStateDispatcher};
use protocol::{Command,Event};
use tracing::error;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        let app = app
            .init_resource::<protocol::Commands>()
            .init_resource::<protocol::Events>()
            .init_resource::<Option<BoxClient>>()
            .init_resource::<Option<ClientStateDispatcher>>()
            .add_system(add_client_state.system())
            .add_system(receive_events.system().label(ProtocolSystem::ReceiveEvents))
            .add_system(
                handle_events
                    .system()
                    .label(ProtocolSystem::HandleEvents)
                    .after(ProtocolSystem::ReceiveEvents)
                    .before(ProtocolSystem::SendCommands),
            )
            .add_system(send_commands.system().label(ProtocolSystem::SendCommands));
        app.add_startup_system(connect_websocket.system());
        #[cfg(target_arch = "wasm32")]
        app.add_system(set_client.system());
        // #[cfg(target_arch = "wasm32")]
        // app.add_system(dial_loop.system());
    }
}

fn add_client_state(
    client: ResMut<Option<BoxClient>>,
    mut state: ResMut<Option<ClientStateDispatcher>>,
) {
    if client.is_some() && state.is_none() {
        *state = Some(Default::default())
    }
}

fn handle_events(
    mut state: ResMut<Option<ClientStateDispatcher>>,
    mut commands: ResMut<protocol::Commands>,
    events: ResMut<protocol::Events>,
) {
    if let Some(ref mut state) = *state {
        let mut context = ClientContext {
            commands: Default::default(),
        };
        for event in events.iter() {
            *state = state.handle(&mut context, &ClientInput::Event(event.clone()));
        }
        *commands = context.commands;
    }
}

fn send_commands(mut client:  ResMut<Option<BoxClient>>, mut commands: ResMut<protocol::Commands>) {
    if let Some(ref mut client) = *client {
        for command in commands.iter() {
            let command = command.clone();
            let len = client.clients.len();
            let rand_int = get_random_int(0,len as i32);
            let mut sender = client.clients.get_mut(rand_int).unwrap().sender();
            if let Command::Nats(b) = command{
              block_on(async move {
                sender.send(b).await.unwrap_or_else(|err| {
                //sender.send(command).await.unwrap_or_else(|err| {
                    error!("{}", err);
                })
              });
            }
        }
        commands.clear();
    }
}
fn receive_events(mut client: ResMut<Option<BoxClient>>, mut events: ResMut<protocol::Events>) {
    if let Some(ref mut client) = *client {
        let len = client.clients.len();
        let rand_int = get_random_int(0,len as i32);
        if let Some(vec) = client.clients.get_mut(rand_int).unwrap().poll_once() {
            for event in vec {
                events.push(event);
            }
        }
    }
}
