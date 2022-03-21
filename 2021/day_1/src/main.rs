use std::fs;

// Answer to Part 1

fn main() {
    let string_data = fs::read_to_string("src/input.txt").expect("file not found");
    let split: Vec<&str> = string_data.split("\r\n").collect();
    let first_value = split[0].parse::<i32>().unwrap();
    let mut previous_value = first_value;
    let mut increase_count = 0;

    for item in split {
        let current_value = item.parse::<i32>().unwrap();
        if current_value > previous_value {
            increase_count += 1;
        }

        previous_value = current_value;
    }

    println!("THIS IS INCREASE COUNT {}", increase_count);
}
