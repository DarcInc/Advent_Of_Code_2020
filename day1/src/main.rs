use std::fs::{File};
use std::io::{BufRead, BufReader, ErrorKind, Error};

struct ExpenseData {
    expenses: [bool; 2021],
    min: usize,
    max: usize,
}

impl ExpenseData {
    fn read(path: &str) -> std::io::Result<ExpenseData> {
        let file = File::open(path)?;
        let mut result = ExpenseData {
            expenses: [false; 2021],
            min: 2021,
            max: 0,
        };

        let mut reader = BufReader::new(file);

        loop {
            let mut line = String::new();

            let size = reader.read_line(&mut line)?;
            if size == 0 {
                break;
            }

            match line.trim_end().parse::<usize>() {
                Ok(idx) => {
                    result.expenses[idx] = true;
                    if idx < result.min {
                        result.min = idx;
                    }

                    if idx > result.max {
                        result.max = idx;
                    }
                },
                Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to parse digit")),
            }
        }

        Ok(result)
    }

    fn find_match_to_amount(&self, amt: usize) -> (usize, usize) {
        for idx in self.min..=self.max {
            if self.expenses[idx] {
                if amt >= idx {
                    if self.expenses[amt-idx] {
                        return (idx, amt-idx);
                    }
                }
            }
        }
        (0, 0)
    }

    fn find_threeway(&self, amt: usize) -> (usize, usize, usize) {
        for idx in self.min..=self.max {
            if self.expenses[idx] {
                let result = amt - idx;

                let candidate = self.find_match_to_amount(result);
                if candidate != (0, 0) {
                    return (idx, candidate.0, candidate.1);
                }
            }
        }
        (0, 0, 0)
    }
}

fn main() {
    match ExpenseData::read("./expense_input.txt") {
        Ok(data) => {
            let matched = data.find_match_to_amount(2020);
            println!("Found {} x {} = {}", matched.0, matched.1, matched.0 * matched.1);

            let threeway = data.find_threeway(2020);
            println!("Found {}, {}, {} = {}", threeway.0, threeway.1, threeway.2, threeway.0 * threeway.1 * threeway.2);
        },
        Err(error) => println!("Yah basic! {}", error),
    }
}
