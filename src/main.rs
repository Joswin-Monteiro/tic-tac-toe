use bevy::prelude::*;

mod window;
use window::WinPlugin;

mod state;
use state::GameState;

mod menu;
use menu::MenuPlugin;

mod game;
use game::BoardPlugin;

fn main() {
    App::new()
        .add_plugins((WinPlugin, MenuPlugin, BoardPlugin))
        .add_systems(Startup, spawn_camera)
        .init_state::<GameState>()
        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
