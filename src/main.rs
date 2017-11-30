fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisble by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }

    // assigning with if

    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of the number is {}", number);

    // while loops

    let mut counter = 3;

    while counter != 0 {
        println!("{}!", counter);
        counter = counter - 1;

    }

    println!("LIFT OFF");

    // looping collections with for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // improved loop, better safety as index won't be out of range and quicker

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // new lift off

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFT OFF");
}
