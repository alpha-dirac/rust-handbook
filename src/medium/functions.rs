pub fn functions_basic() {
    println!("Add -> {}", add(10, 17));
    println!("Subtract -> {}", subtract(1927, 2026));
    let result = tuples_swap(123, 321);
    println!("{} {}", result.0, result.1);

    let user_result = user_tuple(String::from("Jan Koum"), false, 12.8);

    let (name, activity, grade) = (user_result.0, user_result.1, user_result.2);
    println!("Tuple Status -> {} {} {}", name, activity, grade);

    let nothing = do_nothing();
    println!("Status -> {:?}", nothing);
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn add(x: i32, y: i32) -> i32 {
    return x - y;
}

fn tuples_swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn user_tuple(first_name: String, is_active: bool, gpa: f64) -> (String, bool, f64) {
    return (first_name, is_active, gpa);
}

fn do_nothing() -> () {
    return ();
}
