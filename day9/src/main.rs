use std::{fs::File, ops::Range};
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let mut window:HashSet<i32> = HashSet::new();

    let input_file = File::open("./input.txt").expect("Unable to open file");
    let data_reader = BufReader::new(input_file);

    let mut numbers:Vec<i64> = vec![];
    let mut recents:HashSet<i64> = HashSet::new();
    let mut found = false;
    let mut history:Vec<i64> = vec![];
    let mut all_numbers:HashSet<i64> = HashSet::new();
    let mut max_digit = 0;

    for line in data_reader.lines() {
        match line {
            Err(_) => panic!("Failed to read next line!"),
            Ok(raw_line) => {
                let digit:i64 = raw_line.parse().expect("Not a number");
                if numbers.len() == 25 {
                    found = false;
                    for i in 0..25 {
                        let possible = digit - numbers[i];
                        if recents.contains(&possible) {
                            found = true;
                            break;
                        }               
                    }

                    if found {
                        recents.remove(&numbers[0]);
                        numbers.remove(0);
                    } else {
                        max_digit = digit;
                        println!("{} is invalid", digit);
                        break;
                    }
                } 
                history.push(digit);
                all_numbers.insert(digit);
             
                numbers.push(digit);
                recents.insert(digit);
            }
        }
    }

    'outer: for i in 0..history.len() {
        let start_run = i;
        let mut end_run = i + 1;

        'inner: loop {
            let mut sum = 0;
            
            for run_idx in start_run..=end_run {
                sum += history[run_idx];
            }

            if sum == max_digit {
                let mut min = i64::MAX;
                let mut max = i64::MIN;

                for run_idx in start_run..=end_run {
                    if history[run_idx] > max {
                        max = history[run_idx];
                    } 

                    if history[run_idx] < min {
                        min = history[run_idx];
                    }
                }
                println!("Found: {} + {} = {} sum={}", min, max, 
                    min + max, sum);
                break 'outer;
            } else if sum < max_digit {
                end_run += 1;
            } else {
                break 'inner;
            }
        }
    }

}
