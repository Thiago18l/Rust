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
}