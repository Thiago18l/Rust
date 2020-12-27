
/*
 * A scalar type represents a single value. 
 * Rust has four primary scalar types: integers, floating­point numbers, Booleans, and characters. 
 * You may recog­nize these from other programming languages. Let’s jump into how they work in Rust.
 * 
 */
fn main() {
    integers();
    float_point();
    mathematical_operations();
    boolean();
    char_type();
    tuple_type();
}

fn integers() {
    let i : i32 = 1_000;
    println!("I'm a integer: {}", i);
}

fn float_point() {
    let x = 2.0; // f64 double precision IEEE-754
    let y: f32 = 3.0; // f32
    
    println!("{}, {}", x, y);
}

fn mathematical_operations() {
    // addition
    let sum  = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);
}

fn boolean() {
    let t = true;
    let f: bool = false; // explicit type annotation
    println!("{}, {}", t, f);
}

fn char_type() {
    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = '😻';
    println!("{}", _heart_eyed_cat);
}

// Compound types

fn tuple_type() {
    let _tup: (i32, f64, u8) = (500, 6.9, 1);

    let tup = (600, 7.0, 2);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);
}