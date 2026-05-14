pub fn generics_basic() {
    println!("hello world");
    let user_one = User::<String> {
        username: String::from("Omarion"),
    };
    println!("Username status -> {}", user_one.username);
    let user_two = User::<bool> { username: false };
    println!("Username status -> {}", user_two.username);

    let package = Packages {
        item: Item::Inventory(String::from("Empty")),
    };
    println!("Packages Item: {:?}", package.item);

    let bag = BagOfHolding::<i32> { item: None };

    if bag.item.is_none() {
        println!("There's nothing in the bag");
    } else {
        println!("There's something in the bag");
    }

    let bag_two = BagOfHolding::<i32> { item: Some(45) };

    if bag_two.item.is_some() {
        println!("There's something in the bag");
    } else {
        println!("There's nothing in the bag");
    }
}

#[allow(dead_code)]
struct User<T> {
    username: T,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Item {
    Inventory(String),
    None,
}

#[allow(dead_code)]
struct Packages {
    item: Item,
}

#[allow(dead_code)]
struct BagOfHolding<T> {
    item: Option<T>,
}
