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
    loops();
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

fn loops() {
    /*loop {
        println!("infinite")
    } // infinite loop
    */
    let mut number = 3;
    while number != 0 {
        println!("Number is: {}", number);
        number = number - 1;
    }
    let array = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", array[index]);
        index = index + 1;
    }
    
    println!("inside for loop");
    for element in array.iter() {
        println!("{}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}