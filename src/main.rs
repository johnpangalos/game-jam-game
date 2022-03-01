use std::time::Duration;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_tweening::{
    lens::UiPositionLens, Animator, EaseFunction, Tween, TweeningPlugin, TweeningType,
};
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
    // let card_back = asset_server.load("textures/card_back.png");
    // let card_front = asset_server.load("/textures/card_back.png");

    commands.spawn_bundle(UiCameraBundle::default());

    let mut cards = vec![];

    for _ in 0..60 {
        let id = commands
            .spawn()
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

fn update_hand_positions(
    mut commands: Commands,
    query: Query<&Hand, (Changed<Hand>, With<Player>)>,
    style_query: Query<&Style>,
) {
    if query.is_empty() {
        return;
    }

    let Hand(hand) = query.single();

    let gap = 2.0;
    let offset = hand.len() as f32 / 2.0 * gap;

    for (i, e) in hand.iter().enumerate() {
        let style = style_query.get(*e).expect("Style should exist");

        commands.entity(*e).insert(Animator::new(Tween::new(
            EaseFunction::QuinticInOut,
            TweeningType::Once,
            Duration::from_secs(1),
            UiPositionLens {
                start: style.position,
                end: Rect {
                    right: Val::Percent(50.0 + i as f32 * gap - offset),
                    bottom: Val::Percent(5.0),
                    ..Default::default()
                },
            },
        )));
    }
}

fn run_if_clicked<T: Component>(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<T>)>,
) -> ShouldRun {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            return ShouldRun::Yes;
        }
    }

    ShouldRun::No
}

fn press_draw_card_system(
    mut commands: Commands,
    mut query: Query<(&mut Deck, &Hand, Entity), With<Player>>,
    mut player_state: ResMut<State<PlayerState>>,
    asset_server: Res<AssetServer>,
) {
    let (mut deck, Hand(hand), player) = query.single_mut();

    if let Option::Some(card) = deck.cards.pop() {
        let rat = asset_server.load("textures/rat.png");
        commands.entity(card).insert_bundle(ImageBundle {
            image: UiImage(rat),
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(200.0)),
                position: Rect {
                    right: Val::Percent(5.0),
                    bottom: Val::Percent(5.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        });

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
        .add_plugin(TweeningPlugin)
        .add_plugin(DevelopmentPlugin)
        .add_plugin(CustomUiPlugin)
        .add_startup_system(setup)
        .add_system(update_player_state)
        .add_system(update_hand_positions)
        .add_system_set(
            SystemSet::on_update(PlayerState::Draw)
                .with_run_criteria(run_if_clicked::<DrawCardButton>)
                .with_system(press_draw_card_system),
        )
        .add_system_set(
            SystemSet::on_update(PlayerState::Play)
                .with_run_criteria(run_if_clicked::<EndRoundButton>)
                .with_system(press_end_round_system),
        )
        .add_system_set(
            SystemSet::on_exit(PlayerState::Play)
                .with_system(run_cards.label("run_cards"))
                .with_system(update_health::<Player>.after("run_cards"))
                .with_system(update_health::<Computer>.after("run_cards")),
        )
        .run();
}
