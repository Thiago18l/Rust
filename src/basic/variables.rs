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
}