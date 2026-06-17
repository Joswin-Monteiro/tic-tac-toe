use bevy::prelude::*;

pub fn cleanup_sys(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
