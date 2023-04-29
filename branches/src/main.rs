fn count_to(number: u8) {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == number {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
}

fn for_in_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range_loop(from: u8) {
    for number in (1..=from).rev() {
        println!("{number}!");
    }
    println!("LIFT OFF!!!");
}

fn if_false() {
    let number = 3;

    if number != 0 {
        println!("number was somthing other than zero");
    }
}

fn labeled_loop(count_to: u8) {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = count_to;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn multiple_conditions(number: u8) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn step_1() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn ternary_ish(condition: bool) {
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn while_loop(to: u8) {
    let mut number = to;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFT OFF!!!");
}

fn main() {
    step_1();
    if_false();
    multiple_conditions(4);
    multiple_conditions(3);
    multiple_conditions(2);
    multiple_conditions(1);
    ternary_ish(true);
    ternary_ish(false);
    count_to(11);
    labeled_loop(10);
    while_loop(10);
    for_in_loop();
    for_range_loop(3);
}

