use bevy::{ecs::schedule::ShouldRun, prelude::*};
use development::DevelopmentPlugin;

mod development;

trait Card {
    fn strength(&self) -> i32;
}

#[derive(Clone, Copy)]
enum Basic {
    Rat(i32),
}

impl Card for Basic {
    fn strength(&self) -> i32 {
        match self {
            Basic::Rat(s) => s.clone(),
        }
    }
}

struct Round(usize);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Computer;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Hitpoints(i32);

#[derive(Component)]
struct Board([Option<Basic>; 3]);

#[derive(Component)]
struct Hand([Option<Basic>; 10]);

#[derive(Component)]
struct HealthTextComputer(String);

#[derive(Component)]
struct HealthTextPlayer(String);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn()
        .insert(Player)
        .insert(Name("Player".to_string()))
        .insert(Hitpoints(10))
        .insert(Hand([Option::None; 10]))
        .insert(Board([Option::Some(Basic::Rat(1)); 3]));

    commands
        .spawn()
        .insert(Computer)
        .insert(Name("Computer".to_string()))
        .insert(Hitpoints(10))
        .insert(Board([Option::None; 3]));

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "End Round",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 28.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Your Health: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 28.0,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "10".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 28.0,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "My Health: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 28.0,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "10".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 28.0,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(Computer);
}

fn run_cards(
    round: Res<Round>,
    mut p_query: Query<(&Player, &mut Hitpoints, &Board), Without<Computer>>,
    mut c_query: Query<(&Computer, &mut Hitpoints, &Board), Without<Player>>,
) {
    if round.0 == 0 {
        return;
    }

    let (_, mut player_hitpoints, Board(player)) = p_query.single_mut();
    let (_, mut computer_hitpoints, Board(computer)) = c_query.single_mut();

    for column in player.iter().zip(computer.iter()) {
        match column {
            (Option::Some(p), Option::None) => {
                computer_hitpoints.0 = computer_hitpoints.0 - p.strength()
            }
            (Option::None, Option::Some(c)) => {
                player_hitpoints.0 = player_hitpoints.0 - c.strength()
            }
            _ => {}
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

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut round: ResMut<Round>,
) {
    for (interaction, mut color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                round.0 = round.0 + 1;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn run_if_round_changed(round: Res<Round>) -> ShouldRun {
    if round.is_changed() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn main() {
    App::new()
        .insert_resource(Round(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(DevelopmentPlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_round_changed)
                .with_system(run_cards.label("run_cards"))
                .with_system(update_health::<Player>.after("run_cards"))
                .with_system(update_health::<Computer>.after("run_cards")),
        )
        .add_system(button_system)
        .run();
}
