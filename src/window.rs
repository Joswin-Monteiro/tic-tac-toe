use bevy::prelude::*;
use bevy::window::*;

pub struct WinPlugin;

impl Plugin for WinPlugin{
    fn build(&self, app:&mut App){
        app.add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title: "Tic Tac Toe".to_string(),
                name: Some("Tic Tac Toe".to_string()),
                resolution: WindowResolution::new(640, 480),
                ..default()
            }),
            ..default()
        }));
    }
}
