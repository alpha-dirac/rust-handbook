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
}
