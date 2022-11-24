use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        println!("{}", arg);
    }
}

fn cool_println(message: String) {
    // I can't do this in chars. I need to build a String
    // that I can modify and grow into the message.
    let mut chars = message.chars();
    for c in chars {
        for lowercase in 'a'..'z' {
            println!("{}", lowercase);
            if c == lowercase { break; }
        }
        if c.is_lowercase() { break; }
        for uppercase in 'A'..'Z' {
            println!("{}", uppercase);
            if c == uppercase { break; }
        }
        if c.is_uppercase() { break; }
        for number in 0..9 {
            println!("{}", number);
            if c == number { break; }
        }
        if c.is_digit() { break; }
        // TODO special chars for now just add it to the String and
        // print it on the screen
        println!("{}", c);
    }
}
