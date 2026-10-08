use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn load_day(day: &str) -> String{

    let path_string = &("inputs\\day".to_owned()  + day + "\\input.txt");

    let path = Path::new(path_string);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => return s,
    }
}