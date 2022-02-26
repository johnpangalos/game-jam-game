use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

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

#[derive(Component)]
struct FpsText;

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
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn run_cards(
    mut p_query: Query<(&Player, &mut Hitpoints, &Board), Without<Computer>>,
    mut c_query: Query<(&Computer, &mut Hitpoints, &Board), Without<Player>>,
) {
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

fn display_health(query: Query<(&Name, &Hitpoints)>) {
    for (Name(name), Hitpoints(hitpoints)) in query.iter() {
        println!("{:?}: {:?}", name, hitpoints)
    }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(text_update_system)
        .add_system(run_cards.label("run_cards"))
        .add_system(display_health.after("run_cards"))
        .run();
}
