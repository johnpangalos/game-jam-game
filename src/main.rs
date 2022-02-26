use bevy::prelude::*;

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

fn add_cards(mut commands: Commands) {
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

fn main() {
    App::new()
        .add_startup_system(add_cards)
        .add_system(run_cards.label("run_cards"))
        .add_system(display_health.after("run_cards"))
        .run();
}
