fn mutability_example() {
    let mut x = 5;
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing_example() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn tuple_example() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {x} - Destructured");
    println!("The value of y is: {y} - Destructured");
    println!("The value of z is: {z} - Destructured");
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");
}

fn array_example() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
}

fn main() {
    println!("====================");
    mutability_example();
    println!("====================");
    shadowing_example();
    println!("====================");
    tuple_example();
    println!("====================");
    array_example();
    println!("====================");
}

