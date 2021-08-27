use bevy::prelude::*;
use plugin_protocol::ProtocolPlugin;

#[bevy_main]
pub fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ProtocolPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(plugin_wasm_target::WasmTargetPlugin);

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
