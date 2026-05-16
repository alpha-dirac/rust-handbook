pub fn enumerations_basics() {
    let ferris = Creatures {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::None,
    };
    match ferris.species {
        Species::Crab => println!("{} is a Crab", ferris.name),
        Species::Octopus => println!("{} is an Octopus", ferris.name),
        Species::Clam => println!("{} is a Clam", ferris.name),
        Species::Fish => println!("{} is a Fish", ferris.name),
    }

    let plankton = Creatures {
        species: Species::Crab,
        name: String::from("Plankton"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match plankton.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(claws_size, size) => {
                let size_description = match size {
                    Size::Big => "Big",
                    Size::Small => "Small",
                };
                println!(
                    "Ferris is a crab with {} {} claws",
                    claws_size, size_description
                );
            }
            _ => println!("ferris is a crab with other weapon"),
        },
        _ => println!("ferris is some other big animal"),
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
#[derive(Debug)]
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Size {
    Small,
    Big,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

#[allow(dead_code)]
struct Creatures {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}
