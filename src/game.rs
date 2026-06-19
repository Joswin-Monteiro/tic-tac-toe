use bevy::prelude::*;

use crate::state::GameState;

pub struct BoardPlugin;

impl Plugin for BoardPlugin{
    fn build(&self, app:&mut App){
        app
            .init_resource::<BoardState>()
            .init_state::<InGame>()
            .add_systems(OnEnter(InGame::Playing), board_menu.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                (
                    hover_cell.run_if(in_state(InGame::Playing)),
                )
                .run_if(in_state(GameState::InGame)), 
            );
    }
}

#[derive(Component, Default, Clone)]
#[require(Interaction)]
struct Cell(u8);

#[derive(Component, Default, Clone)]
struct Reset;

#[derive(Component, Default, Clone)]
struct Changetext;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player{
    X,
    O,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum InGame {
    #[default]
    Playing,
    Paused,
    GameOver,
}

#[derive(Resource, Debug)]
struct BoardState{
    cells: [Option<Player>; 9],
    cur_player: Player,
    winner: Option<Player>,
}

impl Default for BoardState{
    fn default() -> Self{
        Self{
            cells: [None; 9],
            cur_player: Player::X,
            winner: None,
        }
    }
}

#[derive(Component, Default, Clone)]
struct Board;

fn board_menu(mut commands: Commands){
    commands.spawn_scene(build_board_scene());
}

fn grid_cell(num: u8) -> impl Scene{
    bsn!{
        Cell(num)
        Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
        }
        BorderColor::all(Color::srgb(0.5, 0.5, 0.5))
        BackgroundColor(Color::srgb(0.0, 0.0, 0.0))
        Children [
            Text::new("")
            TextFont {
                font_size: FontSize::Px(50.0)
            }
        ]
    }
}

fn build_board_scene() -> impl Scene {
    let cells = bsn_list![grid_cell(0), grid_cell(1), grid_cell(2), grid_cell(3), grid_cell(4), grid_cell(5), grid_cell(6), grid_cell(7), grid_cell(8)];
    bsn! {
        Board
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,               
            }
        BackgroundColor(Color::srgb(0.0, 0.0, 0.0))
        Children [(
            Node{
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center, 
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::px(3, 100.0)],
                grid_template_rows: vec![RepeatedGridTrack::px(3, 100.0)],
            }         
            Children [{ cells }]
        )]
        Children [
            Node{
                width: percent(100.0),
                height: percent(40.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(10.0)
            }
            Children [
                Changetext
                BackgroundColor(Color::srgb(1.0, 0.0, 0.8))
                Node{
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    //border_radius: BorderRadius::all(px(10.0))
                    width: px(250),
                    height: px(50)
                }
                Children [
                    Text::new("Player X turn")
                    TextFont {
                        font_size: FontSize::Px(20.0)
                    }
                ]
            ]
            Children [
                Reset
                Button
                on(reset_button)
                BackgroundColor(Color::srgb(0.0, 0.0, 0.8))
                Node{
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(10.0))
                    width: px(150),
                    height: px(50)
                }
                Children [
                    Text::new("Reset")
                    TextFont {
                        font_size: FontSize::Px(30.0)
                    }
                ]
            ]
        ]
    }
}

fn hover_cell(
    mut interaction_query: Query<
        (&Interaction, &Cell, &Children, &mut BackgroundColor), 
        (Changed<Interaction>, With<Cell>) 
    >,
    mut text_query: Query<&mut Text>,
    mut changetext_query : Query<&Children, With<Changetext>>,
    mut board_state: ResMut<BoardState>,
    mut ingame_state : ResMut<NextState<InGame>>
) {
    for (interaction, cell, children, mut color) in &mut interaction_query.iter_mut() {
        let cell_index = cell.0 as usize;
        match *interaction {
            Interaction::Pressed => {
                if board_state.cells[cell_index].is_none(){
                    board_state.cells[cell_index] = Some(board_state.cur_player);     

                    let (player_text, player_color) = match board_state.cur_player{
                        Player::X => { 
                            board_state.cur_player = Player::O;
                            ("X", Color::srgb(1.0, 0.0, 0.0))
                        },
                        Player::O => {
                            board_state.cur_player = Player::X;
                            ("O", Color::srgb(0.0, 1.0, 0.0))
                        }
                    };

                    *color = player_color.into();

                    if let Some(&text_entity) = children.first(){
                        if let Ok(mut text) = text_query.get_mut(text_entity){
                            text.0 = player_text.to_string();
                        }
                    }
                    
                    if
                        // Diagonal
                        ((board_state.cells[0] == Some(Player::O)) && (board_state.cells[4] == Some(Player::O)) && (board_state.cells[8] == Some(Player::O))) ||
                        ((board_state.cells[2] == Some(Player::O)) && (board_state.cells[4] == Some(Player::O)) && (board_state.cells[6] == Some(Player::O))) ||
                        // Horizontal
                        ((board_state.cells[0] == Some(Player::O)) && (board_state.cells[1] == Some(Player::O)) && (board_state.cells[2] == Some(Player::O))) ||
                        ((board_state.cells[3] == Some(Player::O)) && (board_state.cells[4] == Some(Player::O)) && (board_state.cells[5] == Some(Player::O))) ||
                        ((board_state.cells[6] == Some(Player::O)) && (board_state.cells[7] == Some(Player::O)) && (board_state.cells[8] == Some(Player::O))) ||
                        // Vertical
                        ((board_state.cells[0] == Some(Player::O)) && (board_state.cells[3] == Some(Player::O)) && (board_state.cells[6] == Some(Player::O))) ||
                        ((board_state.cells[1] == Some(Player::O)) && (board_state.cells[4] == Some(Player::O)) && (board_state.cells[7] == Some(Player::O))) ||
                        ((board_state.cells[2] == Some(Player::O)) && (board_state.cells[5] == Some(Player::O)) && (board_state.cells[8] == Some(Player::O))) 
                    {
                        board_state.winner = Some(Player::O);
                    }else if
                        // Diagonal
                        ((board_state.cells[0] == Some(Player::X)) && (board_state.cells[4] == Some(Player::X)) && (board_state.cells[8] == Some(Player::X))) ||
                        ((board_state.cells[2] == Some(Player::X)) && (board_state.cells[4] == Some(Player::X)) && (board_state.cells[6] == Some(Player::X))) ||
                        // Horizontal                            )                                                                                        
                        ((board_state.cells[0] == Some(Player::X)) && (board_state.cells[1] == Some(Player::X)) && (board_state.cells[2] == Some(Player::X))) ||
                        ((board_state.cells[3] == Some(Player::X)) && (board_state.cells[4] == Some(Player::X)) && (board_state.cells[5] == Some(Player::X))) ||
                        ((board_state.cells[6] == Some(Player::X)) && (board_state.cells[7] == Some(Player::X)) && (board_state.cells[8] == Some(Player::X))) ||
                        // Vertical                              )                                                                                        
                        ((board_state.cells[0] == Some(Player::X)) && (board_state.cells[3] == Some(Player::X)) && (board_state.cells[6] == Some(Player::X))) ||
                        ((board_state.cells[1] == Some(Player::X)) && (board_state.cells[4] == Some(Player::X)) && (board_state.cells[7] == Some(Player::X))) ||
                        ((board_state.cells[2] == Some(Player::X)) && (board_state.cells[5] == Some(Player::X)) && (board_state.cells[8] == Some(Player::X))) 
                    {
                        board_state.winner = Some(Player::X);
                    }

                    let mut text_value = String::from("");
                    
                    if let Some(win) = board_state.winner{
                        ingame_state.set(InGame::GameOver);
                        match win{
                            Player::X => text_value = "Player X won".to_string(),
                            Player::O => text_value = "Player O won".to_string(),
                        }
                    }else{
                        if board_state.cur_player == Player::O{
                            text_value = "Player O turn".to_string();
                        }else{
                            text_value = "Player X turn".to_string();
                        }
                    }

                    if board_state.cells.iter().all(|&cell| cell != None) && board_state.winner.is_none(){
                        text_value = "Game is Draw".to_string();
                        ingame_state.set(InGame::GameOver);
                    }

                    for children in changetext_query{
                        if let Some(&text_entity) = children.first(){
                            if let Ok(mut text) = text_query.get_mut(text_entity){
                                    text.0 = text_value.clone();
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                if board_state.cells[cell_index].is_none(){
                    match board_state.cur_player{
                        Player::X => { 
                           *color = Color::srgb(1.0, 0.0, 0.0).into();
                    },
                        Player::O => {
                           *color = Color::srgb(0.0, 1.0, 0.0).into();
                        }
                    };
                }
            }
            Interaction::None => {
                if board_state.cells[cell_index].is_none(){
                    *color = Color::srgb(0., 0., 0.).into();
                }
            }
        }
    }
}

fn reset_button(
    _click: On<Pointer<Click>>,
    mut board_state: ResMut<BoardState>,
    mut ingame_state: ResMut<NextState<InGame>>,
    mut text_query: Query<&mut Text>,
    mut query: Query<(&Children, &mut BackgroundColor), With<Cell> >,
    changetext_query: Query<&Children, With<Changetext>>,
    ){
    board_state.cells = [None; 9];
    board_state.cur_player = Player::X;
    board_state.winner = None;

    for (children, mut color) in query.iter_mut() {
       if let Some(&text_entity) = children.first(){
           if let Ok(mut text) = text_query.get_mut(text_entity){
               text.0 = "".to_string();
           }
       }
       *color = Color::srgb(0.0, 0.0, 0.0).into();
    }

    for children in &changetext_query {
            if let Some(&text_entity) = children.first() {
                if let Ok(mut text) = text_query.get_mut(text_entity) {
                    text.0 = "Player X turn".to_string();
                }
            }
        }
    ingame_state.set(InGame::Playing);
}

