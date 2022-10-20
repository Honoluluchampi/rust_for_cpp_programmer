fn do_something_for_reference_and_value(_: &i32, _: i32) {
}

fn borrowed_reference() {
    let x = &3;
    let y = *x;
    do_something_for_reference_and_value(x, *x);
    do_something_for_reference_and_value(&y, y);
}

fn multiple_borrowed_reference() {
    let x = 5;
    let y = &x;
    // defining variable with reference construct borrowed reference (not copy)
    let z = y;
    let w = z;
    // calling method and being used by method does not need dereference
    println!("These should all be 5: {} {} {}", *w, y, *z);
}

fn immutable_ref(_: &i32)   { println!("immutable_ref"); }
fn mutable_ref(_: &mut i32) { println!("mutable_ref"); }

fn mutable_reference_is_unique() {
    let x = 5;

    // mutable ref
    let xr = &x;
    immutable_ref(xr);

    let mut x = 5;
    let xr = &x;
    immutable_ref(xr);

    let xr = &mut x;
    mutable_ref(xr);

    // xr is not mutable, but *xr is editable
    *xr *= 4;
    println!("{}", xr);
}

fn variables_can_be_modified_only_by_one_variable() {
    let mut x = 5;
    {
        let _ = &x;
        println!("{}", x);
    }
    {
        let y = &mut x;
        *y = 4;
        x = 4;

        println!("{}", x);
    }
}

fn main() {
    borrowed_reference();
    multiple_borrowed_reference();
    mutable_reference_is_unique();
    variables_can_be_modified_only_by_one_variable();
}