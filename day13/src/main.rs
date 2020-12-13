use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_earliest_start_time(my_start_time: i32, all_start_times: &Vec<i32>) -> (i32, i32) {
    let mut min_mod = i32::MAX;
    let mut bus_number = 0;
    for start_time in all_start_times {
        if my_start_time % start_time < min_mod {
            min_mod = my_start_time % start_time;
            bus_number = *start_time;
        }
    }
    (min_mod, bus_number)
}

fn main() {
    let input_file = File::open("./input.txt").expect("Unable to open input file");
    let mut reader = BufReader::new(input_file);

    let mut start_time = String::new();
    reader.read_line(&mut start_time).expect("Unable to read start time");
    let start_time:i32 = start_time.trim()
        .parse()
        .expect("Unable to parse start time");
    
    let mut schedule = String::new();
    reader.read_line(&mut schedule).expect("Failed to read schedule");
    let schedule = schedule.trim().split(",");
    let mut start_times:Vec<i32> = vec![];

    for raw_time in schedule.into_iter().filter(|x:&&str| x.ne(&"x")) {
        start_times.push(raw_time.parse().expect("Failed to parse start time"));
    }


    let answer = find_earliest_start_time(start_time, &start_times);
    println!("I have to wait {} minutes for bus {} = {}", answer.0, answer.1, answer.0 * answer.1);
}
