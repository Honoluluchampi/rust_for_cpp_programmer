fn if_example(x: i32) -> &'static str {
    let result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn if_without_return(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}

fn while_example() -> () {
    let mut x = 3;
    while x > 0 {
        println!("Current value: {}", x);
        x -= 1;
    }
}

fn for_example(all: &Vec<i32>) -> () {
    // iter style
    for a in all.iter() {
        println!("iter style");
        println!("{}", a);
    }
    // index style
    for i in 0..all.len() {
        println!("index style");
        println!("{}: {}", i, all[i]);
    }
    // enumerate style
    for (i, a) in all.iter().enumerate() {
        println!("enumerate style");
        println!("{}: {}", i, a);
    }
}

fn mut_for_example(all: &mut Vec<i32>) -> () {
    // iter_mut() returns mutable references
    for a in all.iter_mut() {
        *a += *a;
    }
}

fn match_example(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        // y => println!("x is something else {}", y),
        _ => {} // do nothing
    }
}

fn match_expression(x: i32) {
    let msg = match x {
        0 | 1 | 10 => "one of zero, one, or ten",
        y if y < 20 => "less than 20, but not zero, one, or ten",
        y if y == 200 => "200 (but this is not very stylish)",
        _ => "something else"
    }; // semi-colon for let statement
    println!("x is {}", msg);
}

fn main() {
    println!("{}", if_example(5));
    println!("{}", if_without_return(5));

    let mut all: Vec<i32> = vec!(8, 2, 0);

    while_example();
    for_example(&all);
    mut_for_example(&mut all);
    match_example(34);
    match_expression(200);
}