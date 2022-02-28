use bevy::prelude::*;
use dev::DevelopmentPlugin;
use ui::{button, button_text, description, CustomUiPlugin, DescriptionOptions};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum PlayerState {
    Draw,
    Play,
}

mod dev;
mod ui;

struct Round(usize);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Computer;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Card {
    name: String,
    strength: i32,
}

#[derive(Component)]
struct Hitpoints(i32);

#[derive(Component)]
struct Board([Option<Entity>; 3]);

#[derive(Component)]
struct Hand(Vec<Entity>);

#[derive(Component)]
struct Deck {
    draws: i32,
    cards: Vec<Entity>,
}

#[derive(Component)]
struct HandCard {}

#[derive(Component)]
struct PlayerStateText;

#[derive(Component)]
struct EndRoundButton;

#[derive(Component)]
struct DrawCardButton;

#[derive(Bundle)]
struct CardBundle {
    card: Card,

    #[bundle]
    sprite: SpriteBundle,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium = asset_server.load("fonts/FiraMono-Medium.ttf");
    let card_back = asset_server.load("textures/card_back.png");
    // let card_front = asset_server.load("/textures/card_back.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let mut cards = vec![];
    for _ in 0..60 {
        let id = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, -215.0, 0.0),
                texture: card_back.clone(),
                ..Default::default()
            })
            .id();

        cards.push(id);
    }

    commands.spawn_bundle((
        Player,
        Name("Player".to_string()),
        Hitpoints(10),
        Hand(vec![]),
        Board([Option::None; 3]),
        Deck { draws: 1, cards },
    ));

    commands.spawn_bundle((
        Computer,
        Name("Computer".to_string()),
        Hitpoints(10),
        Board([Option::None; 3]),
    ));

    commands
        .spawn_bundle(button(Rect {
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..Default::default()
        }))
        .with_children(button_text("End Round", bold.clone()))
        .insert(EndRoundButton);

    commands
        .spawn_bundle(button(Rect {
            bottom: Val::Px(200.0),
            right: Val::Px(10.0),
            ..Default::default()
        }))
        .with_children(button_text("Draw Card", bold.clone()))
        .insert(DrawCardButton);

    commands
        .spawn_bundle(description(
            "Your Health: ",
            "10",
            DescriptionOptions {
                position: Rect {
                    bottom: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                title_font: bold.clone(),
                definition_font: medium.clone(),
            },
        ))
        .insert(Player);

    commands
        .spawn_bundle(description(
            "My Health: ",
            "10",
            DescriptionOptions {
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                title_font: bold.clone(),
                definition_font: medium.clone(),
            },
        ))
        .insert(Computer);

    commands
        .spawn_bundle(description(
            "Player State: ",
            "Draw",
            DescriptionOptions {
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(200.0),
                    ..Default::default()
                },
                title_font: bold.clone(),
                definition_font: medium.clone(),
            },
        ))
        .insert(PlayerStateText);
}

// fn draw_cards(
//     query: Query<&Deck, With<Player>>,
//     sprites_query: Query<&Sprite>,
//     asset_server: Res<AssetServer>,
// ) {
//     let deck = query.single();
//     let sprites = sprites_query.iter().collect::<Vec<&Sprite>>();
// }

fn run_cards(
    round: Res<Round>,
    mut p_query: Query<(&mut Hitpoints, &Board), (With<Player>, Without<Computer>)>,
    mut c_query: Query<(&mut Hitpoints, &Board), (With<Computer>, Without<Player>)>,
    cards_query: Query<&Card>,
) {
    if round.0 == 0 {
        return;
    }

    let (mut player_hitpoints, Board(player)) = p_query.single_mut();
    let (mut computer_hitpoints, Board(computer)) = c_query.single_mut();

    for column in player.iter().zip(computer.iter()) {
        match column {
            (Option::Some(p), Option::None) => {
                let card = cards_query.get(*p).expect("Should exist");
                computer_hitpoints.0 = computer_hitpoints.0 - card.strength
            }
            (Option::None, Option::Some(c)) => {
                let card = cards_query.get(*c).expect("Should exist");
                player_hitpoints.0 = player_hitpoints.0 - card.strength;
            }
            _ => {}
        }
    }
}

fn press_end_round_system(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<EndRoundButton>),
    >,
    mut round: ResMut<Round>,
    mut player_state: ResMut<State<PlayerState>>,
) {
    for (interaction, _children) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            round.0 = round.0 + 1;
            player_state
                .set(PlayerState::Draw)
                .expect("This should work");
            return;
        }
    }
}

fn gen_card(position: Rect<Val>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(200.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            position,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn press_draw_card_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<DrawCardButton>),
    >,
    mut query: Query<(&mut Deck, &mut Hand), With<Player>>,
    mut player_state: ResMut<State<PlayerState>>,
) {
    let (mut deck, mut hand) = query.single_mut();
    for (interaction, _children) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            if let Option::Some(card) = deck.cards.pop() {
                commands
                    .spawn_bundle(gen_card(Rect {
                        bottom: Val::Px(50.0),
                        left: Val::Px(15.0 + 165.0 * hand.0.len() as f32),
                        ..Default::default()
                    }))
                    .insert(HandCard {});
                hand.0.push(card);
                deck.draws = deck.draws - 1;
            }

            if deck.draws <= 0 {
                deck.draws = 1;
                player_state
                    .set(PlayerState::Play)
                    .expect("This should work");
                return;
            }
        }
    }
}

fn update_health<T: Component>(
    query: Query<&Hitpoints, With<T>>,
    mut text_query: Query<&mut Text, With<T>>,
) {
    let Hitpoints(hitpoints) = query.single();
    let mut text = text_query.single_mut();
    text.sections[1].value = format!("{:?}", hitpoints);
}

fn update_player_state(
    mut text_query: Query<&mut Text, With<PlayerStateText>>,
    player_state: Res<State<PlayerState>>,
) {
    let mut text = text_query.single_mut();

    text.sections[1].value = format!("{:?}", player_state.current())
}

fn main() {
    App::new()
        .insert_resource(Round(0))
        .add_state(PlayerState::Draw)
        .add_plugins(DefaultPlugins)
        .add_plugin(DevelopmentPlugin)
        .add_plugin(CustomUiPlugin)
        .add_startup_system(setup)
        .add_system(update_player_state)
        .add_system_set(SystemSet::on_update(PlayerState::Draw).with_system(press_draw_card_system))
        .add_system_set(SystemSet::on_update(PlayerState::Play).with_system(press_end_round_system))
        .add_system_set(
            SystemSet::on_exit(PlayerState::Play)
                .with_system(run_cards.label("run_cards"))
                .with_system(update_health::<Player>.after("run_cards"))
                .with_system(update_health::<Computer>.after("run_cards")),
        )
        .run();
}
