use regex::Regex;

pub fn solve(input: &str) {
    solve_a(input);
    solve_b(input);
}

fn solve_a(input: &str) {
    println!("part_a");
    
    let regex = Regex::new(r"(?<lower>\d+)-(?<upper>\d+) (?<character>.+?): (?<password>(?s)[^\n]*)").unwrap();
    let entries: Vec<(&str, &str, &str, &str)> = regex.captures_iter(input)
        .map(|entry|{
            let (_, [lower, upper, character, password]) = entry.extract();
            (lower, upper, character, password)
        }).collect();

    let mut total_quantity = 0;

    for i in entries {
        let lower_bound: i32 = i.0.parse().unwrap();
        let upper_bound: i32 = i.1.parse().unwrap();
        let required_char: char = i.2.parse().unwrap();
        let password: &str = i.3;
        let mut count = 0;
        for _char in password.chars(){
            if _char == required_char {
                count = count + 1; 
            }
        }
        if count >=  lower_bound && count <=  upper_bound {
            total_quantity = total_quantity +1;
        }
    }
    println!("{}", total_quantity);
}


fn solve_b(input: &str) {
     println!("part_b");
    
    let regex = Regex::new(r"(?<lower>\d+)-(?<upper>\d+) (?<character>.+?): (?<password>(?s)[^\n]*)").unwrap();
    let entries: Vec<(&str, &str, &str, &str)> = regex.captures_iter(input)
        .map(|entry|{
            let (_, [lower, upper, character, password]) = entry.extract();
            (lower, upper, character, password)
        }).collect();

    let mut total_quantity = 0;

    for i in entries {
        let first_char_index: usize = i.0.parse::<usize>().unwrap() - 1;
        let second_char_index: usize = i.1.parse::<usize>().unwrap() - 1;
        let required_char: char = i.2.parse().unwrap();
        let password: &str = i.3;

        if password.chars().nth(first_char_index) == password.chars().nth(second_char_index) {
            continue;
        }
        if (password.chars().nth(first_char_index) == Some(required_char)) || (password.chars().nth(second_char_index) == Some(required_char)){
            total_quantity = total_quantity + 1;
        }
    }
    println!("{}", total_quantity);
}
