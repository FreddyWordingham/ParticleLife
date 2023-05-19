use bevy::{prelude::*, window::close_on_esc};
use rand::prelude::*;

// == Settings ==

// == Main ==
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SimulationPlugin)
        .run();
}

// == Plugins ==
struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(close_on_esc);
    }
}
