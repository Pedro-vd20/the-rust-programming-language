use std::io;

fn print_is_divisible(number: i32) {
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


fn loop_and_return() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break counter * 2   // returns expression here
        }
        println!("{} - I'm in a loop!", counter);
    }
}


fn loop_labels() {
    // break / continue always apply to innermost loop unless loop is labelled
    // then we can specify what to break / continue
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

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


fn for_loops() {
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("The value is {element}");
    }

    // Ranges can be created with (x..y)
    for number in 1..10 {
        println!("I can count to 10!: {number}");
    }

    // .rev to reverse
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("BLASTOFF");
}


fn main() {
    let mut number = String::new();
    println!("Choose a number");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");

    let number: i32 = number.trim()
        .parse()
        .expect("Input is not a number");
    print_is_divisible(number);

    let condition = number < 5;
    if condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // conditional assignment
    let transform: f64 = if condition {number as f64 * 2.0} else {number as f64 / 2.0};
    println!("Condition evaluated to {}", transform);

    println!("Looping, {}", loop_and_return());

    loop_labels();
    for_loops();
}