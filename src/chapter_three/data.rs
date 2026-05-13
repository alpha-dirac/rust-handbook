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
}

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapons: String,
}
