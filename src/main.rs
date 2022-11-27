use std::env;
use std::{thread,time};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut argc: usize = 1; // print out previous argv

    for arg in &argv[1..] {
        let chars = arg.chars();
        let mut pos: usize = 0; // keep track of which character your on

        // each character of the given arg
        for c in chars {

            // print through all chars as we echo
            for i in 32..126 {
                let ichar: char = char::from_u32(i).unwrap();
                // print previous args
                for arg in &argv[1..argc] { eprint!("{} ", arg) }
                eprintln!("{}{}", &arg.as_str()[0..pos], ichar);
                thread::sleep(time::Duration::from_millis(5));
                if c == ichar { pos += 1; break; }
            }

        }

        // include the printed arg 
        argc += 1;
    }

    for arg in &argv[1..argv.len()] { print!("{} ", arg) }
    println!();
}
