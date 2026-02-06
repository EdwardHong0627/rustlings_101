// Characters (`char`)
fn is_alphabetic(c: char) -> bool {
    if c.is_alphabetic() {
        true
    } else if c.is_numeric() {
        false
    } else {
        false
    }
}
fn main() {
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

    // TODO: Analogous to the example before, declare a variable called `your_character`
    // below with your favorite character.
    // Try a letter, try a digit (in single quotes), try a special character, try a character
    // from a different language than your own, try an emoji 😉
    // let your_character = '';
    let your_character = 'a';
    let digit = '1';
    let special = '!';
    let emoji = '😉';
    println!("{}", is_alphabetic(your_character));
    println!("{}", is_alphabetic(digit));
    println!("{}", is_alphabetic(special));
    println!("{}", is_alphabetic(emoji));
}
