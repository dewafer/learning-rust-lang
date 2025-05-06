use std::collections::HashMap;
use rand::Rng;

fn convert_to_pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let first_char = word.chars().next().unwrap();
            if "aeiouAEIOU".contains(first_char) {
                format!("{}-hay", word)
            } else {
                let mut chars = word.chars();
                let first = chars.next().unwrap();
                format!("{}-{}ay", chars.collect::<String>(), first)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(5..=27); // Randomly choose the length between 5 and 27
    let numbers: Vec<i32> = (0..length).map(|_| rng.gen_range(1..=10)).collect(); // Generate random numbers based on the chosen length
    println!("Generated numbers: {:?}", numbers);

    let (median, mode) = calculate_median_and_mode(&numbers);
    println!("Median: {}", median);
    println!("Mode: {}", mode);

    let sentence = "first apple";
    let pig_latin = convert_to_pig_latin(sentence);
    println!("Pig Latin: {}", pig_latin);
}

fn calculate_median_and_mode(numbers: &Vec<i32>) -> (f64, i32) {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    println!("Sorted numbers: {:?}", sorted_numbers);

    // Calculate median
    let len = sorted_numbers.len();
    let median = if len % 2 == 0 {
        (sorted_numbers[len / 2 - 1] + sorted_numbers[len / 2]) as f64 / 2.0
    } else {
        sorted_numbers[len / 2] as f64
    };

    // Calculate mode
    let mut occurrences = HashMap::new();
    for &num in numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }
    let mode = occurrences.into_iter().max_by_key(|&(_, count)| count).unwrap().0;

    (median, mode)
}
