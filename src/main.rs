use std::collections::HashMap;
use std::io;

fn main() {
    let mut list = Vec::new();
    loop {
        println!("Please input a space-separated list of numbers.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let mut error = false;
        for number in input.trim().split_whitespace() {
            match number.parse() {
                Ok(num) => list.push(num),
                Err(_) => {
                    println!("Could not parse number: {}", number);
                    list = Vec::new();
                    error = true;
                    continue;
                },
            }
        }
        if list.len() == 0 {
            println!("Empty list");
            error = true;
        }
        if !error {
            println!("Please input a space-separated list of numbers.");
            break;
        }
    }
    list.sort();
    println!("Number list: {:?}\nMean: {}\nMedian: {}\nMode: {}",
             list, mean(&list), median(&list), mode(&list));
}

fn mean(v: &Vec<i32>) -> i32 {
    let count = v.len();
    let mut total = 0;
    for val in v {
        total += val;
    }
    total / count as i32
}

fn median(v: &Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    v[mid]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut num_map = HashMap::new();
    for num in v {
        let count = num_map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut greatest_val = 0;
    let mut greatest_count = 0;
    for (val, count) in num_map {
        if count > greatest_count {
            greatest_val = *val;
            greatest_count = count;
        }
    }
    greatest_val
}
