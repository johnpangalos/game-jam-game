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

fn setup(mut commands: Commands) {
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
}

fn run_cards(
    round: Res<Round>,
    mut p_query: Query<(&Player, &mut Hitpoints, &Board), Without<Computer>>,
    mut c_query: Query<(&Computer, &mut Hitpoints, &Board), Without<Player>>,
) {
    if !round.is_changed() {
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

fn display_health(round: Res<Round>, query: Query<(&Name, &Hitpoints)>) {
    if !round.is_changed() {
        return;
    }

    for (Name(name), Hitpoints(hitpoints)) in query.iter() {
        println!("{:?}: {:?}", name, hitpoints)
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut round: ResMut<Round>) {
    if keys.just_pressed(KeyCode::Space) {
        round.0 = round.0 + 1;
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
                .with_system(display_health.after("run_cards")),
        )
        .add_system(keyboard_input)
        .run();
}
