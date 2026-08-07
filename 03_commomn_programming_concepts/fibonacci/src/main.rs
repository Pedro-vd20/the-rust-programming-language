use std::io;

fn collect_user_int() -> u32 {
    loop {
        let mut user_in = String::new();
        println!("Enter a Fibonacci index: ");
        io::stdin()
            .read_line(&mut user_in)
            .expect("Failed to read input");

        let user_in: u32 = match user_in.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse input as a number: {user_in}");
                continue;
            }
        };

        return user_in;
    }
}

// 1, 1, 2, 3, 5, 8, 13, 21...
fn compute_nth_fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    
    let mut prev = [1, 1];
    let mut nth = 1;
    let mut counter = 2;
    while counter < n {
        nth = prev[0] + prev[1];
        prev[1] = prev[0];
        prev[0] = nth;
        counter += 1;
    }

    nth
}

fn main() {
    let nth_index = collect_user_int();
    println!(
        "The {nth_index}th number in the Fibonacci sequence is {}",
        compute_nth_fibonacci(nth_index)
    );
}
