use std::io;

/*
 * DATA TYPES
 * Scalar:
 *  - Integer
 *      - 8bit: i8, u8
 *      - 16bit: i16, u16
 *      - 32bit: i32, u32       DEFAULT
 *      - 64bit: i64, u64
 *      - 128bit: i128, u128
 *      - arch: isize, usize 
 *  - Floating point
 *      - f32
 *      - f64                   DEFAULT
 *  - Booleans
 *  - Character: supports unicode, 4 bytes
 * Compound:
 *  - Tuple: fixed size grouping of values of maybe many types
 *  - Array: 
 */
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    // constants MUST be type annotated even if the type can be inferred
// Fun fact: RUST compiler will evaluate some basic operations like multiplication


fn constants_and_mutables() {
    // Variables and mutability
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This would cause an error because x is immutable
    // println!("The value of x is: {}", x);

    // Shadowing
    let x = x + 1; // This is allowed because we are shadowing the previous x
    println!("The value of x after shadowing is: {}", x);

    // scoping prevents keeps the previous x alive 
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is unchanged: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // This is allowed because y is mutable
    println!("The new value of y is: {}", y);

    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
}


fn shadowing_and_mutability() {
    let x = "5";
    println!("The value of x (string) is: {}", x);
    let x: u32 = x.parse().expect("Not a number!"); // shadowing allows us to change the type of x
    println!("Shadowing lets us change the type of a variable. The value of x (number) is: {}", x);

    let mut y = "5";
    println!("The value of y (string) is: {}", y);
    // y = y.parse().expect("Not a number!"); // This would cause an error because y is mutable but its type is still &str
    // println!("The value of y (number) is: {}", y);
}


fn compound_types() {
    let tuple: (bool, f64, &str) = (true, 3.14, "hello there");
    println!("The value of the second element (index 1) is {}", tuple.1);

    // arrays allocate on the STACK!!!!! because they are fixed size
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of the third element (index 2) is {}", array[2]);
    
    // arrays can be initialized with same value for all elements
    let same_val_array = [1; 10]; // an array of length 10 where all elements are 1
    println!("New array with same value for all elements: {:?}", same_val_array);

    let mut user_input = String::new();
    println!("Please enter an index to access the 5th element of the array (0-4):");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input: usize = user_input.trim().parse().expect("Failed to parse number");
    // Rust does checking for out of bounds access at runtime and will panic to preserve memory safety
    println!("The 5th element of the array is {}", array[user_input]);
}


fn main() {
    constants_and_mutables();
    shadowing_and_mutability();
    compound_types();
}
