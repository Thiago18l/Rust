fn main() {
    println!("My number: {}", 140);
    arithmetics();
    float_();
}
fn arithmetics() {
    println!("The sum is: {}", 80 + 34);
    println!("{} + {} = {}", 34, 80, 80 + 34);
    println!("{}", (23 - 6) % 5 + 20 * 30 / (3 + 4)) // output is 87
}

fn float_() {
    println!("The sum is: {}", 80.3 + 34.8);
    println!("{} {}", -12 % 10, -1.2 % 1.);
}

