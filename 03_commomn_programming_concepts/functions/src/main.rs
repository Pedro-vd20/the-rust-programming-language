fn another_function(x: i32) {
    println!("Another function");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}


fn expressions_and_statements() -> i32 {
    // statements are instructions, don't return
    let x = 3;
    let y = {   // start new scope
        let x = 4;
        x + 1
    };
    // x is back to being 3
    println!("Values x: {}, y: {}", x, y);

    // expressions evaluate and return
    x * 2   // no semicolon = expression, if expression last statement, then it's returned
}

fn main() {
    println!("Hello, world!");
    another_function(10);
    print_labeled_measurement(5, 'm');
    println!("Computed expressions: {}", expressions_and_statements())
}

