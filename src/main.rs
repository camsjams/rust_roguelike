extern crate rand;
#[macro_use]
extern crate text_io;

mod character;
use rand::Rng;
use std::io;

fn main() {
    println!(
        "=== Welcome to {} v{} ====",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    );

    let characters: Vec<&character::Player> = [
        character:Character::new("Cleric", 7, 5, 6, 7),
        character:Character::new("Warrior", 10, 5, 5, 5),
        character:Character::new("Hunter", 5, 7, 7, 6),
        character:Character::new("Wizard", 3, 10, 5, 7),
        character:Character::new("Thief", 4, 5, 6, 10),
    ];

    let luck_amount = rand::thread_rng().gen_range(2, 11);

    println!("You enter the Ephemeral Plane of Creation ...");
    println!("Please input your character name.");

    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    let character_name = input_text.trim();

    println!("Please select your character type:");
    for (i, elem) in characters.iter().enumerate() {
        print!("\n{}. {}\n\n", i, elem.info());
    }

    let mut character_index: usize = characters.len();
    while character_index > characters.len() {
        character_index = read!();
    }

    play(characters[character_index]);
}

fn play(player: character::Character) {
    println!("Welcome aboard {} the {}!", player.name, player.class);
}
