use bevy::prelude::*;
use plugin_protocol::ProtocolPlugin;
use log::Level;
#[bevy_main]
pub fn main() {
    
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ProtocolPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(plugin_wasm_target::WasmTargetPlugin);
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(Level::Debug);
    #[cfg(feature = "bevy_mod_debugdump")]
    {
        std::fs::write(
            "target/schedule_graph.dot",
            bevy_mod_debugdump::schedule_graph::schedule_graph_dot(&app.app.schedule),
        );
        std::process::exit(0);
    }

    app.run();
}
