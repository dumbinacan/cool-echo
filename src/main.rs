use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        cool_println(arg);
    }
}

fn cool_println(message: String) {
    // I can't do this in chars. I need to build a String
    // that I can modify and grow into the message.
    let chars = message.chars();
    // TODO make sure copy contains empty string
    //      before pushing
    let mut copy = String::new();
    for c in chars {
        for lowercase in 'a'..'z' {
            print!("{}", copy);
            println!("{}", lowercase);
            if c == lowercase {
                copy.push(lowercase);
                break;
            }
        }
        if c.is_lowercase() { break; }
        for uppercase in 'A'..'Z' {
            print!("{}", copy);
            println!("{}", uppercase);
            if c == uppercase {
                copy.push(uppercase);
                break;
            }
        }
        if c.is_uppercase() { break; }
        for number in '0'..'9' {
            println!("{}", number);
            if c == number {
                copy.push(number);
                break;
            }
        }
        if c.is_numeric() { break; }
        // TODO special chars for now just add it to the String and
        // print it on the screen
        println!("{}\n We hit the bottom of loop next char please!", c);
    }
}
