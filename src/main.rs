use std::collections::HashMap;

// Min numbers count required for input
const MIN_NUMBERS: usize = 3;

fn main() {
    println!("Insert an odd number of numbers (min {} numbers): ", MIN_NUMBERS);

    // get numbers from user's input
    let data: String = get_input_data();

    // build the numbers vector and the hash map with occurrences
    let (numbers, numbers_occurrence) = prepare_numbers(data);

    // calculate and print program results
    println!("Sorted vector: {:?}", numbers);
    println!("Median: {:.1}", get_median(numbers));
    println!("Mode: {:?}", get_mode(numbers_occurrence));
}

fn get_median(numbers: Vec<i32>) -> f32 {
    let middle: usize = numbers.len() / 2;

    if numbers.len() % 2 == 0 {
        (numbers[middle - 1] as f32 + numbers[middle] as f32) / 2.0
    } else {
        numbers[middle] as f32
    }
}

fn get_mode(numbers_map: HashMap<i32, u8>) -> i32 {
    numbers_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn get_input_data() -> String {
    let mut data: String = String::new();
    std::io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line!");

    data
}

fn prepare_numbers(data: String) -> (Vec<i32>, HashMap<i32, u8>) {
    let mut numbers: Vec<i32> = Vec::new();
    let mut numbers_occurrence: HashMap<i32, u8> = HashMap::new();

    for word in data.trim().split_whitespace() {
        let number: i32 = word.trim().parse().expect("Please type a positive number!");

        numbers.push(number);
        *numbers_occurrence.entry(number).or_insert(0) += 1;
    }
    numbers.sort();

    if numbers.len() < MIN_NUMBERS {
        panic!("Min {} numbers required!", MIN_NUMBERS);
    }

    (numbers, numbers_occurrence)
}
