pub fn datastructures_basic() {
    let state = String::from("Hello world");
    println!("{} is {} characters long", state, state.len());

    let sea_creature = SeaCreature {
        animal_type: String::from("Mammals"),
        name: String::from("Horse"),
        arms: 2,
        legs: 2,
        weapons: String::from("Galloping"),
    };
    println!("Animal Type -> {}", sea_creature.animal_type);
    println!("Name -> {}", sea_creature.name);
    println!("Arms -> {}", sea_creature.arms);
    println!("Legs -> {}", sea_creature.legs);
    println!("Weapons -> {}", sea_creature.weapons);

    let location = Location(300000, String::from("USA"), true, 1776);
    let (pop, name, is_strong, year) = (location.0, location.1, location.2, location.3);
    println!("Status: {} {} {} {}", pop, name, is_strong, year);

    let ferris = Animal {
        species: Species::Fish,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("Claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is a crab", ferris.name),
        Species::Octopus => println!("{} is an octopus", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
    }
}

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapons: String,
}

struct Location(i32, String, bool, i32);

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

struct Animal {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}
