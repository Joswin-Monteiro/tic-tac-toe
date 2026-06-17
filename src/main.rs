use bevy::prelude::*;

mod window;
use window::WinPlugin;

mod state;
use state::GameState;

fn main() {
    App::new()
        .add_plugins(WinPlugin)
        .init_state::<GameState>()
        .run();
}
