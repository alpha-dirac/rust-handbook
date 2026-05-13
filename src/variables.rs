pub fn variable_basics() {
    let x = 13;
    println!("{}", x);

    let x: f64 = 3.14159;
    println!("{}", x);

    let x;
    x = 0;
    println!("{}", x);

    let is_active = false;
    println!("Activity status -> {}", is_active);

    let is_active = "Not active";
    println!("Updated activity -> {}", is_active);

    let mut age = 27;
    println!("Age -> {}", age);

    age = -27;
    println!("Negative age -> {}", age);

    let groups = (20, false, 1.5, "mark", 'S');
    println!(
        "Tuple Groups -> {} {} {} {} {}",
        groups.0, groups.1, groups.2, groups.3, groups.4
    );

    let items: [i32; 4] = [1, 2, 3, 4];
    for item in items.iter() {
        println!("Items -> {}", item);
    }
}
