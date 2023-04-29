fn expression_example() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");

    another_function(5, 'h');
    expression_example();

    let xx = plus_one(5);
    println!("The value of xx is: {xx}");
}
