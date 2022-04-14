#![allow(warnings, unused)]

pub mod cache;

use ansi_term::Color::*;
use indoc::indoc;
use std::env;
use std::process;
use std::time::Instant;


fn main() {
    //begin timing program execution
    let start = Instant::now();

    //argument variables
    let mut use_2lvl_cache = false;
    let mut cache1_capacity = 0;
    let mut cache2_capacity = 0;
    let mut filepath = String::new();

    //test metric variables
    let mut num_refs1 = 0;
    let mut num_refs2 = 0;
    let mut num_hits1 = 0;
    let mut num_hits2 = 0;
    let mut num_hits_total = 0;
    let mut hit_ratio1 = 0.0;
    let mut hit_ratio2 = 0.0;
    let mut hit_ratio_global = 0.0;

    //echo user input
    let args:Vec<String> = env::args().collect();
    println!("You entered:");
    for (i,arg) in args.iter().enumerate() {
        println!("\tfield {}: {}",i,arg);
    }
    println!();

    //process args
    if !(4<=args.len() && args.len()<=5) {
        print_usage_and_exit("Incorrect number of arguments provided");
    }
    cache1_capacity = args[2].parse::<u32>().unwrap_or_else(|ignored| {print_usage_and_exit("Field 2 must be a positive integer"); 0});
    match args[1].as_str() {
        "1" => {
            use_2lvl_cache = false;
            filepath = args[2].clone();
        },
        "2" => {
            use_2lvl_cache = true;
            cache2_capacity = args[3].parse::<u32>().unwrap_or_else(|ignored| {print_usage_and_exit("Field 3 must be a positive integer"); 0});
            filepath = args[3].clone();
        },
        _ => {print_usage_and_exit("Field 1 must be a 1 or 2");}
    }

    //display total run time
    let elapsed = start.elapsed();
    println!("Program execution took {}.{:0<3} seconds", elapsed.as_secs(), elapsed.subsec_millis());
}


fn print_usage_and_exit(message:&str) {
    print!("{}\n{}", Red.paint(message), indoc!{"

        Usage:
            cargo run 1 <1st-level_cache_size> <input_textfile_name>
            cargo run 2 <1st-level_cache_size> <2nd-level_cache_size> <input_textfile_name>

    "});
    process::exit(1);
}