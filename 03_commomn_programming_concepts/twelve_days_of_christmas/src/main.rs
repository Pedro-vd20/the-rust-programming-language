const GIFTS: [&str; 12] = [
    "A partridge in a pear tree",
    "two turtle doves",
    "Three French hens",
    "Four calling brids",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eight",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

fn main() {
    for i in 0..12 {
        println!("On the {} day of Christmas\nMy true love gave to me", DAYS[i]);
        for day in (0..i+1).rev() {
            if day == 0 && i > 0 {
                print!("And ")
            }
            println!("{}", GIFTS[day]);
        }
        println!();
    }
}
