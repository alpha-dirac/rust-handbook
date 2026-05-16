pub fn enumerations_basics() {
    let ferris = Creatures {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("Claw"),
    };
    match ferris.species {
        Species::Crab => println!("{} is a Crab", ferris.name),
        Species::Octopus => println!("{} is an Octopus", ferris.name),
        Species::Clam => println!("{} is a Clam", ferris.name),
        Species::Fish => println!("{} is a Fish", ferris.name),
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Species {
    Crab,
    Fish,
    Octopus,
    Clam,
}

#[allow(dead_code)]
struct Creatures {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}
