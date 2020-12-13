use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader,Result};

struct BoardingPass {
    original_pass: String,
    row_number: i32,
    column_number: i32,
}

fn partition(boundary: char, range: &(i32, i32), bounds: &(char, char)) -> (i32, i32) {
    if boundary == bounds.0 {
        (range.0, (range.1 - range.0) / 2 + range.0)
    } else if boundary == bounds.1 {
        ((range.1 - range.0) / 2 + range.0 + 1, range.1)
    } else {
        panic!("Boundary does not exist in the given bounds");
    }
}

fn get_row_number(sections: &str) -> i32 {
    let mut bounds = (0, 127);
    let boundaries = ('F', 'B');
    for section in sections.chars() {
        bounds = partition(section, &bounds, &boundaries);
    }

    bounds.0
}

fn get_column_number(sections: &str) -> i32 {
    let mut bounds = (0, 7);
    let boundaries = ('L', 'R');
    for section in sections.chars() {
        bounds = partition(section, &bounds, &boundaries);
    }

    bounds.0
}

impl BoardingPass {
    fn from(raw_string: &str) -> Self {
        let splitting_expression = Regex::new("([FB]+)([RL]+)").expect("Failed to parse splitting expression");
        let captures = splitting_expression.captures(raw_string).unwrap();
        let row_number = get_row_number(&captures[1]);
        let column_number = get_column_number(&captures[2]);

        BoardingPass{original_pass: String::from(""), row_number: row_number, column_number: column_number}
    }

    fn load_from_file(path: &str) -> Result<Vec<Self>> {
        let data_file = File::open(path)?;
        let data_reader = BufReader::new(data_file);

        let mut result: Vec<BoardingPass> = vec![];
        for line in data_reader.lines() {
            match line {
                Ok(raw_text) => {
                    result.push(BoardingPass::from(&raw_text));
                }, 
                Err(error) => {
                    return Err(error.into());
                }
            }
        }

        Ok(result)
    }

    fn seat_id(&self) -> i32 {
        self.row_number * 8 + self.column_number
    }
}

fn max_seat_id(boarding_passes: &Vec<BoardingPass>) -> i32 {
    let mut max_seat_id = 0;
    for bp in boarding_passes {
        if bp.seat_id() > max_seat_id {
            max_seat_id = bp.seat_id();
        }
    }
    max_seat_id
}

fn main() {
    let boarding_passes = BoardingPass::load_from_file("./input.txt").expect("Failed to load boarding passes");
    let max_seat_number = max_seat_id(&boarding_passes) as usize;
    println!("Max seat id = {}", max_seat_number);

    let remaining_passes = (&boarding_passes).into_iter().filter(|bp| bp.row_number != 0 && bp.row_number != 127);
    
    let mut filled_seats:[char; 1024] = ['O'; 1024];
    for pass in remaining_passes {
        filled_seats[pass.seat_id() as usize] = 'X';
    }

    for k in 11..(max_seat_id(&boarding_passes) as usize) {
        if filled_seats[k - 2] == 'X' && filled_seats[k - 1] == 'O' && filled_seats[k] == 'X' {
            println!("Found your seat: {}", k - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranges() {
        let test_data = &[
            ('F', (0, 127), ('F', 'B'), (0, 63)),
            ('B', (0, 127), ('F', 'B'), (64, 127)),
            ('F', (64, 127), ('F', 'B'), (64, 95)),
            ('B', (64, 127), ('F', 'B'), (96, 127)),
            ('F', (64, 95), ('F', 'B'), (64, 79)),
            ('B', (64, 95), ('F', 'B'), (80, 95)),
            ('F', (80, 95), ('F', 'B'), (80, 87)),
            ('B', (80, 95), ('F', 'B'), (88, 95)),
            ('F', (80, 87), ('F', 'B'), (80, 83)),
            ('B', (80, 87), ('F', 'B'), (84, 87)),
            ('F', (80, 83), ('F', 'B'), (80, 81)),
            ('B', (80, 83), ('F', 'B'), (82, 83)),
            ('F', (80, 81), ('F', 'B'), (80, 80)),
            ('B', (80, 81), ('F', 'B'), (81, 81)),
        ];

        for datum in test_data {
            let answer = partition(datum.0, &datum.1, &datum.2);
            let expected = datum.3;
            assert_eq!(answer.0, expected.0);
            assert_eq!(answer.1, expected.1)
        }
    }

    #[test]
    fn test_get_row_number() {
        let test_cases = &[
            ("BFFFBBF", 70),
            ("FFFBBBF", 14),
            ("BBFFBBF", 102),
        ];

        for test_case in test_cases {
            let result = get_row_number(test_case.0);
            assert_eq!(test_case.1, result);
        }
    }

    #[test]
    fn test_get_column_number() {
        let test_cases = &[
            ("RRR", 7),
            ("RLL", 4),
        ];

        for test_case in test_cases {
            let result = get_column_number(test_case.0);
            assert_eq!(test_case.1, result);
        }
    }

    #[test]
    fn test_boarding_pass_from() {
        let test_cases = &[
            ("BFFFBBFRRR", 70, 7),
            ("FFFBBBFRRR", 14, 7),
            ("BBFFBBFRLL", 102, 4),
        ];

        for test_case in test_cases {
            let bp = BoardingPass::from(test_case.0);

            assert_eq!(test_case.1, bp.row_number);
            assert_eq!(test_case.2, bp.column_number);
        }
    }

    #[test]
    fn test_seat_id() {
        let bp = BoardingPass::from("BFFFBBFRRR");
        assert_eq!(567, bp.seat_id());
    }
}