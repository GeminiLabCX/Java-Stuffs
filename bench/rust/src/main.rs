use annoy_rs::*;
use std::env;
use std::time;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let dim = args[1].parse().unwrap();
    let size = args[2].parse::<u64>().unwrap();
    let n_result = args[3].parse().unwrap();
   