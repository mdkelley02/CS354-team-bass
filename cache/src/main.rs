pub mod cache;

use ansi_term::Color::*;
use indoc::indoc;
use std::env;
use std::process;
use std::time::Instant;

fn main() {
    //begin timing program execution
    let start = Instant::now();

    //echo user input
    let args:Vec<String> = env::args().collect();
    println!("You entered:");
    for (i,arg) in args.iter().enumerate() {
        println!("\tfield {}: {}", i, arg);
    }
    println!();

    //validate args
    if !(4..=5).contains(&(args.len() as i32)) {
        eprintln!("{}",Red.paint("Incorrect number of arguments provided"));
        print_usage_and_exit();
    }

    //display total run time
    let elapsed = start.elapsed();
    println!("Program execution took {}.{} seconds", elapsed.as_secs(), elapsed.subsec_millis());
}

fn print_usage_and_exit() {
    print!("{}",indoc! {"

        Usage:
            cargo run 1 <cache_size> <input_textfile_name>
            cargo run 2 <1st-level_cache_size> <2nd-level_cache_size> <input_textfile_name>

    "});
    process::exit(1);
}