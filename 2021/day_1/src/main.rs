use std::fs;

// Answer to Part 1
fn part_1() {
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

fn part_2() {
    let string_data = fs::read_to_string("src/input.txt").expect("file not found");
    let split: Vec<&str> = string_data.split("\r\n").collect();
    let mut increase_count = 0;

    for (index, _) in split.iter().enumerate() {
        let a1 = split.get(index);
        let a2 = split.get(index + 1);
        let a3 = split.get(index + 2);

        let b1 = split.get(index + 1);
        let b2 = split.get(index + 2);
        let b3 = split.get(index + 3);

        if a1.is_some() && a2.is_some() && a3.is_some() && b1.is_some() && b2.is_some() && b3.is_some() {
            let c1 = a1.unwrap().parse::<i32>().unwrap();
            let c2 = a2.unwrap().parse::<i32>().unwrap();
            let c3 = a3.unwrap().parse::<i32>().unwrap();

            let d1 = b1.unwrap().parse::<i32>().unwrap();
            let d2 = b2.unwrap().parse::<i32>().unwrap();
            let d3 = b3.unwrap().parse::<i32>().unwrap();

            let window_1 = c1 + c2 + c3;
            let window_2 = d1 + d2 + d3;

            if window_2 > window_1 {
                increase_count += 1;
            }
        }
    }

    println!("THIS IS INCREASE COUNT {}", increase_count);
}

fn main() {
    part_1();
    part_2();
}
