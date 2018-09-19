extern crate rand;
#[macro_use]
extern crate text_io;

use rand::Rng;
use std::io;

fn main() {
    println!(
        "=== Welcome to {} v{} ====",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    );

    const CLASSES: [&str; 5] = ["Cleric", "Warrior", "Hunter", "Wizard", "Thief"];

    let luck_amount = rand::thread_rng().gen_range(2, 11);

    println!("You enter the Ephemeral Plane of Creation ...");
    println!("Please input your character name.");

    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    let character_name = input_text.trim();

    print_classes(CLASSES);

    let mut character_type: usize = 10;
    while character_type == 10 {
        character_type = read!();
    }

    play(character_name, luck_amount, CLASSES[character_type]);
}

fn play(name: &str, luck: i32, character_type: &str) {
    println!("Welcome aboard {} the {}!", name, character_type);
    println!("You have {} luck points.", luck);
}

fn print_classes(classes: [&str]) {
    println!("Please select your character type:");
    for (i, elem) in classes.iter_mut().enumerate() {
        print!("\n{}. {}\n\n", i + 1, elem);
    }
}
