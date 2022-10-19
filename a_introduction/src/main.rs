fn foo(_: &'static str) -> &'static str {
    return "world";
}

fn main() {
    let world = "world";
    println!("Hello, {}! {} {} ", world, 9, 2.5);
    println!("Hello, {}!", foo("bar"));
}
