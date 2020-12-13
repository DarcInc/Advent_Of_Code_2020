use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
/// 
/// if there's a delta of 3 volts - there only one option.
/// Let's say you have 110, 111, 112, 113, and 116
/// You can colnnect 
/// 113 - 1 option
///      113 - 116
/// 112 - 1 option (1 options for 113)
///      112 - 113 - 116 
/// 111 - 2 options 1 + 1 (options for 112 + options for 113)
///      111 - 112 - 113 - 116 
///      111 - 113 - 116 
/// 110 - 4 options 2 + 1 + 1 (options for 111, + options for 112, + options for 113)
///      110 - 111 - 112 - 113 - 116,
///      110 - 111 - 113 - 116 
///      110 - 112 -113 -116, 
///      110 - 113 -116
/// 108 - 6 options (4 options for 110, + 2options for 111) 
///      108 - 110 - 111 - 112 - 113 - 116,
///      108 - 110 - 111 - 113 - 116,
///      108 - 110 - 112 - 113 - 116,
///      108 - 110 - 113 - 116,
///      108 - 111 - 112 - 113 - 116,
///      108 - 111 - 113 - 116
///
/// the options for node n are the sum of options the next 1-3 nodes it could possibly connect to.

fn main() {
    let input_file = File::open("./input.txt").unwrap();
    let buffer = BufReader::new(input_file);

    let mut adapter_joltages:Vec<i32> = vec![];
    adapter_joltages.push(0);
    for line in buffer.lines() {
        let line = line.unwrap();
        let digit:i32 = line.parse().unwrap();

        adapter_joltages.push(digit);
    }

    adapter_joltages.sort();
    adapter_joltages.push(adapter_joltages.last().unwrap() + 3);

    let mut one_delta = 0;
    let mut three_delta = 0;

    for i in 1..adapter_joltages.len() {
        match adapter_joltages[i] - adapter_joltages[i - 1] {
            1 => one_delta += 1,
            3 => three_delta += 1,
            _ => {} 
        }
    }

    println!("one: {} three: {} onexthree: {}", one_delta, three_delta, one_delta * three_delta);

    let mut adapter_index = adapter_joltages.len() - 2;
    let mut options:HashMap<i32, i64> = HashMap::new();
    options.insert(adapter_joltages[adapter_index], 1); // adapter index is -2

    println!("{} has {} options", adapter_joltages[adapter_index], 1);
    loop {
        adapter_index -= 1; // adapter index is -3
        let mut total_options = 0;
        let mut followers = adapter_index + 1; // follower is -2

        loop {
            if adapter_joltages[followers] - adapter_joltages[adapter_index] > 3 {
                break;
            } 
            if followers > adapter_joltages.len() - 2 {
                break;
            }

            total_options += options[&adapter_joltages[followers]];

            followers += 1;
        }

        println!("{} has {} options", adapter_joltages[adapter_index], total_options);
        options.insert(adapter_joltages[adapter_index], total_options);

        if adapter_index == 0 {
            break;
        } 
    }

    println!("{}", options[&0]);
}
