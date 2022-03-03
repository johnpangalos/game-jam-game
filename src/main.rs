use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType, EasingsPlugin};

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
struct HandOrigin;

#[derive(Component)]
struct DeckOrigin;

#[derive(Component)]
struct PlayerStateText;

#[derive(Component)]
struct EndRoundButton;

#[derive(Component)]
struct DrawCardButton;

#[derive(Component)]
struct Camera2D;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium = asset_server.load("fonts/FiraMono-Medium.ttf");
    let rat = asset_server.load("textures/Rat.png");

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Camera2D);
    commands.spawn_bundle(UiCameraBundle::default());

    let mut cards = vec![];

    for _ in 0..60 {
        let id = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(225.0, 300.0)),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                texture: rat.clone(),
                ..Default::default()
            })
            .insert(Card {
                name: "Rat".to_string(),
                strength: 1,
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
        .spawn_bundle(NodeBundle {
            style: Style {
                position: Rect {
                    right: Val::Percent(50.0),
                    bottom: Val::Percent(0.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HandOrigin);

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position: Rect {
                    right: Val::Percent(5.0),
                    bottom: Val::Percent(15.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DeckOrigin);

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
            (Option::Some(e), Option::None) => {
                let card = cards_query.get(*e).expect("Should exist");
                computer_hitpoints.0 = computer_hitpoints.0 - card.strength
            }
            (Option::None, Option::Some(e)) => {
                let card = cards_query.get(*e).expect("Should exist");
                player_hitpoints.0 = player_hitpoints.0 - card.strength;
            }
            _ => {}
        }
    }
}

fn move_camera_system(mut camera: Query<&mut Transform, With<Camera2D>>, windows: Res<Windows>) {
    let window = windows.get_primary().expect("Window to exist");

    let mut transform = camera.single_mut();

    transform.translation.x = window.width() / 2.0;
    transform.translation.y = window.height() / 2.0;
}

fn press_end_round_system(
    In(result): In<Result<bool, ()>>,
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<EndRoundButton>),
    >,
    mut round: ResMut<Round>,
    mut player_state: ResMut<State<PlayerState>>,
) {
    if Result::Ok(false) == result {
        return;
    }

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

fn button_is_clicked<T: Component>(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<T>)>,
) -> Result<bool, ()> {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            return Result::Ok(true);
        }
    }

    Result::Ok(false)
}

fn deck_system(
    origin_query: Query<&Transform, With<DeckOrigin>>,
    query: Query<&Deck, With<Player>>,
    mut cards_query: Query<(&mut Transform, &mut Visibility), (With<Card>, Without<DeckOrigin>)>,
) {
    let Deck { cards, .. } = query.single();
    let origin = origin_query.single();

    for card in cards {
        let (mut transform, mut visibility) = cards_query.get_mut(*card).expect("Card to exist");
        transform.translation = origin.translation.clone();
        visibility.is_visible = false;
    }
}

fn hand_system(
    mut commands: Commands,
    mut cards_query: Query<(&Transform, &mut Visibility), (With<Card>, Without<HandOrigin>)>,
    query: Query<&Hand, (Changed<Hand>, With<Player>)>,
    hand_origin_query: Query<&Transform, With<HandOrigin>>,
) {
    if query.is_empty() {
        return;
    }

    let hand_origin = hand_origin_query.single();

    let Hand(hand) = query.single();

    let card_space = PI / 13.0;
    let offset = match hand.len() {
        0..=1 => 0.0,
        _ => card_space * hand.len() as f32 - 0.2 * hand.len() as f32,
    };
    let spacing = match hand.len() {
        0..=1 => 0.0,
        _ => offset / (hand.len() - 1) as f32,
    };

    let radius = 2000.0;
    let height = 80.0;
    let card_tilt = 0.1;

    for (i, e) in hand.iter().enumerate() {
        let (transform, mut visibility) = cards_query.get_mut(*e).expect("Transform should exist");

        visibility.is_visible = true;
        let a = i as f32 * spacing - offset / 2.0 + (PI / 2.0);
        let x = a.cos() * radius;

        let y = -radius + a.sin() * radius;

        let offset_a = PI / 2.0 - a;

        commands.entity(*e).insert(transform.ease_to(
            Transform {
                translation: Vec3::new(-x, y + height, 0.0) + hand_origin.translation,
                rotation: Quat::from_rotation_z(match offset_a {
                    x if x < 0.001 && x > -0.001 => 0.0,
                    _ => offset_a + offset_a.signum() * card_tilt,
                }),
                ..Default::default()
            },
            EaseFunction::QuadraticIn,
            EasingType::Once {
                duration: Duration::from_millis(300),
            },
        ));
    }
}

fn press_draw_card_system(
    In(result): In<Result<bool, ()>>,
    mut commands: Commands,
    mut query: Query<(&mut Deck, &Hand, Entity), With<Player>>,
    mut player_state: ResMut<State<PlayerState>>,
) {
    if Result::Ok(false) == result {
        return;
    }

    let (mut deck, Hand(hand), player) = query.single_mut();

    if let Option::Some(card) = deck.cards.pop() {
        let mut next = hand.clone();
        next.push(card);
        commands.entity(player).insert(Hand(next));

        deck.draws = deck.draws - 1;
    }

    if deck.draws <= 0 {
        deck.draws = 1;
        player_state
            .set(PlayerState::Play)
            .expect("This should work");
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

fn player_state_system(
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
        .add_plugin(EasingsPlugin)
        .add_plugin(DevelopmentPlugin)
        .add_plugin(CustomUiPlugin)
        .add_startup_system(setup)
        .add_system(move_camera_system)
        .add_system(player_state_system)
        .add_system(hand_system)
        .add_system(deck_system)
        .add_system_set(
            SystemSet::on_update(PlayerState::Draw)
                .with_system(button_is_clicked::<DrawCardButton>.chain(press_draw_card_system)),
        )
        .add_system_set(
            SystemSet::on_update(PlayerState::Play)
                .with_system(button_is_clicked::<EndRoundButton>.chain(press_end_round_system)),
        )
        .add_system_set(
            SystemSet::on_exit(PlayerState::Play)
                .with_system(run_cards.label("run_cards"))
                .with_system(update_health::<Player>.after("run_cards"))
                .with_system(update_health::<Computer>.after("run_cards")),
        )
        .run();
}
