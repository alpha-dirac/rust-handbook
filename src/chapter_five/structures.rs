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

    let database = Database(20, String::from("Relational"), true);
    let (rows, db_type, is_persistent) = (database.0, database.1, database.2);
    println!(
        "Rows: {} Database Type: {} Is Persistent: {}",
        rows, db_type, is_persistent
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

#[allow(dead_code)]
struct Database(i32, String, bool);
