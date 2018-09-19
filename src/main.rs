extern crate rand;
#[macro_use]
extern crate text_io;

mod character;
mod computer;

use character::Player;
use computer::Enemy;
use rand::Rng;
use std::io;

fn main() {
    println!(
        "=== Welcome to {} v{} ====",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    );

    let characters: [character::Character; 5] = [
        character::Character::new("".to_string(), "Cleric".to_string(), 7, 5, 6, 7),
        character::Character::new("".to_string(), "Warrior".to_string(), 10, 5, 5, 5),
        character::Character::new("".to_string(), "Hunter".to_string(), 5, 7, 7, 6),
        character::Character::new("".to_string(), "Wizard".to_string(), 3, 10, 5, 7),
        character::Character::new("".to_string(), "Thief".to_string(), 4, 5, 6, 10),
    ];

    let _luck_amount = rand::thread_rng().gen_range(2, 6);

    println!("You enter the Ephemeral Plane of Creation ...");
    println!("Please input your character name.");

    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    let _character_name = input_text.trim();

    println!("Please select your character type:");
    for (i, elem) in characters.iter().enumerate() {
        print!("\n{}. {}\n\n", i + 1, elem.info());
    }

    let mut character_index: usize = 100;
    while character_index > characters.len() {
        character_index = read!();
    }

    let mut player =
        characters[character_index - 1].select(_character_name.to_string(), _luck_amount);

    play(&mut player);
}

fn play(player: &mut character::Character) {
    println!(
        "=== Welcome to RRL {} the {}! ====\n",
        player.name, player.class
    );
    println!("Your unique stats: {}", player.stats());
    let mut enemy = computer::Computer::new(1, 16);

    while player.health > 0 {
        println!(
            "\n====\nYour health: {}\n(a)ttack or (d)odge\n",
            player.health
        );

        let _action = enemy.action();
        let _cpu_action = rand::thread_rng().gen_range(_action.0, _action.1);

        let mut input_text = String::new();

        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");
        let _player_move = input_text.trim();

        let mut _player_action = 0;

        if _player_move == "a" {
            _player_action = player.attack();
            println!("> You attacked with {} attack power!", _player_action);
            println!("> CPU dodged with {} dodge power!", _cpu_action);
        } else {
            _player_action = player.dodge();
            println!("> CPU attacked with {} dodge power!", _cpu_action);
            println!("> You dodged with {} dodge power!", _player_action);
        }

        if _player_action > _cpu_action {
            let heal = (_player_action - _cpu_action) / 2;
            println!("\nFlawless Victory (you heal {})", heal);
            player.heal(heal)
        } else {
            let dmg = _cpu_action - _player_action;
            println!("\nTis but a flesh wound (you take {} damage)", dmg);
            player.damage(dmg)
        }

        if player.health > 0 {
            enemy.level_up()();
        }
    }

    println!(
        "\n\n=== Game Over! ====\nYour soul has lost connection to this land.\n\n--Final Stats--Player:\n{}\nEnemy:\n{}",
        player.stats(),
        enemy.stats()
    );
}
