fn take_ownership(some_string: String) {
    println!("`{some_string}` is now mine");
}

fn return_ownership(some_string: String) -> String {
    println!("I will be returning ownership of `{some_string}`");
    some_string
}

fn borrow_value(some_string: &String) -> usize {
    println!("Don't worry, I only borrowed `{some_string}`");
    // some_string.push_str("MORE WORDS");     // We borrowed some_string, it's not ours to modify
    some_string.len()
}

fn borrow_and_mutate(some_string: &mut String) {
    // There can only ever be 1 mutable reference
    some_string.push_str(" 1, 2, 3");
    println!("I have modified `{some_string}` without owning it!");
}

// fn first_word(s: &String) -> &str {     // `&str` indicates slice
fn first_word(s: &str) -> &str      // using `&str` instead of `&String` is preferable as it allows for both slices AND Strings.
    // // Implementation 1: return index of first space
    // let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    // s.len()      // this approach returns a value fully independent of the string and its lifetime

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    println!("Hello, world!");

    {       // creates a block of scope, variables in here only exist inside this block
        let s = "hello";
        println!("{s}");
    } // s stops existing

    let mut s = String::from("hello"); // different s (different scope)
    s.push_str(" there!"); // append
    println!("{s}");

    let mut s2 = s; // Now s2 is the owner of the data held by s
    // println!("{s}");   // will fail because s is now invalid

    s2 = String::from("Test");   // This causes `drop()` to be called on the initial value of s2
    let s3 = s2.clone();        // deep copy
    println!("{s2}");           // we can still use s2

    s2 = return_ownership(s2);  // s2 is returned and can still be used
    borrow_value(&s2);
    borrow_and_mutate(&mut s2);
    take_ownership(s2);    // and now s2 is gone

    let b1 = &s3; // immutable reference
    let b2 = &s3; // immutable, we can have multiple immutable references

    // let b3 = &mut s3; // FAILS multiple immutable references -> can't have a mutable one

    let x = 3;
    let y = x;       // automatic deep copy for stack variables
}
