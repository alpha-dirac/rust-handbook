use std::result;

pub fn conditional_basics() {
    let status = "single";

    if status == "single" {
        println!("The user is single");
    } else {
        println!("The user is married");
    }

    let x = 42;

    if x <= 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("Greater than 42");
    }

    let mut infinite_loops = 0;

    loop {
        infinite_loops += 1;
        if infinite_loops == 42 {
            break;
        }
    }
    println!("Loops -> {}", infinite_loops);

    let mut x = 52;
    while x != 52 {
        x += 1;
    }
    println!("X is  {}", x);

    for item in 0..=10 {
        if item % 2 == 0 {
            println!("{} is even number", item)
        } else {
            println!("{} is odd number", item);
        }
    }

    let x = 49;

    match x {
        0 => {
            println!("found zero");
        }
        1 | 2 => {
            println!("found 1 or 2");
        }
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        matched_num @ 10..=100 => {
            println!("Found {} number between 10 to 100", matched_num);
        }
        _ => {
            println!("found something else");
        }
    }

    let mut x = 0;

    let value = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", value);
    println!("from function: {}", iterator_logic());
}

fn iterator_logic() -> i32 {
    let iterator = 42;

    let value = if iterator < 42 { -1 } else { 1 };
    println!("from if: {}", value);

    let food = "Hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        _ => "is not working",
    };
    println!("Identifying food: {}", result);

    let value = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", value);

    value + 4
}
