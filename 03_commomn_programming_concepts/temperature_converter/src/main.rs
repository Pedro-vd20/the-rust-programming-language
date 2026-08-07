use std::io;

enum TempMode {
    CToF,
    FToC,
}

fn celsius_to_fahrenheit(c_temp: f64) -> f64 {
    (9.0 / 5.0) * c_temp + 32.0
}

fn fahrenheit_to_celsius(f_temp: f64) -> f64 {
    (5.0 / 9.0) * (f_temp - 32.0)
}

fn select_mode() -> TempMode {
    loop {
        let mut user_in = String::new();
        println!("Select mode:");
        io::stdin()
            .read_line(&mut user_in)
            .expect("Failed to read user input");

        let user_in: u32 = match user_in.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unable to parse input as number: {user_in}\nPlease select one of the options.");
                continue;
            }
        };

        if user_in == 1 {
            return TempMode::CToF;
        }
        else if user_in == 2 {
            return TempMode::FToC;
        }

        println!("Invalid option, can only select 1 / 2");
    }
}


fn input_temp() -> f64 {
    loop {
        let mut user_in = String::new();
        println!("Input temperature");
        io::stdin()
            .read_line(&mut user_in)
            .expect("Failed to read user input");

        let user_in: f64 = match user_in.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unable to parse input as number: {user_in}");
                continue;
            }
        };

        return user_in;
    }
}


fn convert_temp(temp: f64, mode: TempMode) -> f64 {
    match mode {
        TempMode::CToF => celsius_to_fahrenheit(temp),
        TempMode::FToC => fahrenheit_to_celsius(temp),
    }
}


fn main() {
    println!(
        "Welcome to Temperature converter!\n \
        Select a mode (choose the number only):\n \
        1. Convert Celcius to Fahrenheit\n \
        2. Convert Fahrenheit to Celcius"
    );
    let mode = select_mode(); 
    let temp = input_temp();

    let conversion = convert_temp(temp, mode);
    println!("Converted temperature: {conversion}");
}
