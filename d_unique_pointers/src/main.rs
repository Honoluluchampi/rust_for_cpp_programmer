fn unique_pointer_example() -> () {
    let x = Box::new(75); // Box::new() is like c++'s new
    println!("'x' points to {}", *x);
}

fn unique_pointer_mutability() {
    let _x = Box::new(75); // unused variable should have the prefix '_'
    let y = Box::new(42);
    // x = y; not allowed
    // *x = 43; not allowed
    let mut x = Box::new(75);
    println!("x : {}", x);
    x = y;
    println!("x : {}", x);
    // method calls automatically dereference
    println!("*x : {}", *x);
    *x = 43;
    println!("*x : {}", *x);
}

fn returning_u_ptr() -> Box<i32> {
    Box::new(75)
}

fn returning_u_ptr_example() {
    let y = returning_u_ptr();
    println!("{}", y);
}

fn main() {
    unique_pointer_example();
    unique_pointer_mutability();
    returning_u_ptr_example();
}
