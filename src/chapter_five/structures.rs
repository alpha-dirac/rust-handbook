pub fn structures_basics() {
    let result = say_hello("Mark Swoll", "Deploy");
    println!("Message result -> {:?}", result);

    let ferris = SeaCreature {
        animal_type: String::from("Crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("Claw"),
    };
    println!(
        "{} is a {}. They have {} arms, {} legs, and a {}",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
}

fn say_hello(username: &str, message: &str) {
    println!(
        "Mr {} message is {} -> ",
        username.to_string(),
        message.to_string()
    );
}

#[allow(dead_code)]
struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}
