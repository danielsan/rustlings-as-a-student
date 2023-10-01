// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)

// I AM NOT_DONE
// use rand::prelude::*;

fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // let x: u8 = random();
    let x: u8 = 35;
    let your_character: char = char::from_u32(x.into()).unwrap();// Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical: [{}] -> {}", your_character, x);
    } else if your_character.is_numeric() {
        println!("Numerical: [{}] -> {}", your_character, x);
    } else {
        println!("Neither alphabetic nor numeric: [{}] -> {}", your_character, x);
    }
}
