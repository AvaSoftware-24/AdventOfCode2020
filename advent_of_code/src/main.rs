use std::env;

mod utils;
mod day_1;
mod day_2;

use crate::utils::load_inputs::load_day;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input = load_day(&args[1]);

    match args[1].as_str(){
    "1"=>day_1::solve(&input),
    "2"=>day_2::solve(&input),
    &_ => println!("please enter a valid day")
    }
}