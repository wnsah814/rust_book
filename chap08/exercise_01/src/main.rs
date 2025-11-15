use std::collections::HashMap;

fn main() {
    let v = vec![1, 3, 4, 3, 2, 1];

    println!("Original vector: {:?}", v);

    match calc_median(&v) {
        Some(median) => println!("Median: {}", median),
        None => println!("Median: Vector is empty"),
    }

    match calc_mode(&v) {
        Some(mode) => println!("Mode: {}", mode),
        None => println!("Mode: Vector is empty"),
    }
}

fn calc_median(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    let mut sorted = v.clone();
    sorted.sort();

    let len = sorted.len();
    let median = if len % 2 == 0 {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2
    } else {
        sorted[len / 2]
    };

    Some(median)
}

fn calc_mode(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    let mut occurrences = HashMap::new();

    for &val in v {
        *occurrences.entry(val).or_insert(0) += 1;
    }

    // iterator chaining, clousure
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
}