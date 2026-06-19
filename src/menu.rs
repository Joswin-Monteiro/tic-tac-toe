use bevy::prelude::*;
use crate::state::GameState;

pub struct MenuPlugin;
impl Plugin for MenuPlugin{
    fn build(&self, app:&mut App){
        app
            .add_systems(OnEnter(GameState::MainMenu), setup_menu)
            //.add_systems(Update, button_system)
            .add_systems(OnExit(GameState::InGame), cleanup);
    }
}
#[derive(Component, Default, Clone)]
pub struct MainMenuRoot;

#[derive(Component, Default, Clone)] struct PlayButton;

#[derive(Component, Default, Clone)]
struct QuitButton;

fn setup_menu(mut commands: Commands){
    commands.spawn_scene(
        bsn!{
            MainMenuRoot
            Node{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0)
            }
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0))
            Children [
                (
                    Node
                    Text::new("Tic Tac Toe")
                    TextFont{
                        font_size: FontSize::Px(40.0)
                    }
                ),
                (
                    Button
                    PlayButton
                    on(play_click)
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                    }
                    BackgroundColor(Color::srgb(0.0, 0.0, 0.8))
                    Children [
                        Text::new("Play")
                        TextFont {
                            font_size: FontSize::Px(30.0)
                        }
                    ]
                ),
                (
                    Button
                    on(exit_click)
                    QuitButton
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                    }
                    BackgroundColor(Color::srgb(0.0, 0.0, 0.8))
                    Children [
                        Text::new("Exit")
                        TextFont {
                            font_size: FontSize::Px(30.0)
                        }
                    ]
                ),  
            ]
        }
    );
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayButton>,
            Option<&QuitButton>,
        ),
        Changed<Interaction>,
    >,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, play_btn, quit_btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if play_btn.is_some() {
                    *color = Color::srgb(0.0, 1.0, 0.0).into(); 
                    state.set(GameState::InGame);
                } else if quit_btn.is_some() {
                    *color = Color::srgb(1.0, 0.0, 0.0).into();
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.4, 0.4, 0.4).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn play_click(_click: On<Pointer<Click>>, mut state: ResMut<NextState<GameState>>){
    state.set(GameState::InGame);
}

fn exit_click(_click: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>){
    exit.write(AppExit::Success);
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
