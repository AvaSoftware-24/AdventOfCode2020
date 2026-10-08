use std::collections::HashSet;

pub fn solve(input: &str) {
    solve_a(input);
    solve_b(input);
}

fn solve_a(input: &str) {
    println!("part_a");
    let mut read_values: HashSet<i32> = HashSet::new();
    //itterate through, calc the number that would be required for this specific pair, if it is not present in the set add it.
    for entry in input.lines() {
        let parsed_value = entry.parse().unwrap();
        if read_values.contains(&(2020 - parsed_value)){
            let entry_a = parsed_value;
            let entry_b = 2020 - entry_a;
            let product = entry_a * entry_b;
            println!("entry_a: {}\nentry_b: {}\nproduct: {}\n", entry_a, entry_b, product);
            break;
        }
        read_values.insert(parsed_value);
    }
}

fn solve_b(input: &str) {
    println!("part_b");
    //sort array

    let mut read_values: HashSet<i32> = HashSet::new();

    //here's some lambda so you're happy Emi :)
    let mut sorted_entries: Vec<i32>  = input
        .lines()
        .map(|entry|{
            let value: i32 = entry.parse().unwrap();
            read_values.insert(value);
            value
        })
        .collect();

    sorted_entries.sort();

    //ittr smallest > largest
    for (i, &outer_value) in sorted_entries.iter().enumerate() {
        let target_sum = 2020 - outer_value;

        //ittr largest > smallest, trimming entries that are no longer valid
        for &inner_value in sorted_entries[i + 1..].iter().rev() {
            // Trim the largest value, and ensure the itterator no longer tests against it
            if target_sum < inner_value {
                continue;
            }
            let search_value = target_sum - inner_value;
            // search if the value is in the set
            if read_values.contains(&search_value){
                // correct path
                let product = outer_value * inner_value * search_value;
                println!(
                    "entry_a: {}\n\
                    entry_b: {}\n\
                    entry_c: {}\n\
                    product: {}\n",
                    outer_value, inner_value, search_value, product
                );
                return;
            }
        }
    }
}