fn main() {
    let number = 12;
    let other_number = 53;
    print!("{}", number + other_number);

    print!("{} {}\n", number, 47);
    mutable();
}
fn mutable() {
    let mut number = 12;
    print!("before: {}\n", number);
    number = 20;
    print!("after: {}\n", number);
    values();
}

fn values() {
    let mut x = 5; // the keyword mut make the variable mutable to change the content inside it
    println!("{}", x);
    x = 20;
    println!("{}", x);
    constants();
    shadow();
    strings();
}

fn constants() {
    const MAX_POINTS: u32 = 100_000;    
    println!("{}", MAX_POINTS);
}

fn shadow() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x: {}", x);
}

fn strings() {
    let spaces = "   ";
    let spaces = spaces.len(); // the use of let is important to create a other variable with same name as the before
    println!("{}", spaces);
}