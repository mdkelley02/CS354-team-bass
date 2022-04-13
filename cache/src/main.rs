pub mod cache;

use std::env;
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
    

    //display total run time
    let elapsed = start.elapsed();
    println!("Program execution took {}.{} seconds", elapsed.as_secs(), elapsed.subsec_millis());
}
