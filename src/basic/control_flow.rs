fn main() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    let six: i32 = 6;
    if six % 4 == 0 {
        println!("Number is divisible by 4");
    } else if six % 3 == 0 {
        println!("Number is divisible by 3");
    } else if six % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    conditional();
}

fn conditional() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("{}", number);
}