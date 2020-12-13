use std::io::{prelude::*, BufReader};
use regex;

#[derive(Clone, Copy)]
struct Policy {
    min: i32,
    max: i32,
    character: char,
}

impl Policy {
    fn new(min: i32, max: i32, character: char) -> Self {
        Policy {
            min,
            max,
            character
        }
    }

    fn evalute(&self, test_string: &str) -> bool {
        //let mut count = 0;
        //for c in test_string.chars() {
        //    if c == self.character {
        //        count += 1;
        //    }
        //}
        //count <= self.max && count >= self.min
        let first_character = test_string.chars().nth((self.min - 1) as usize).unwrap_or_default();
        let second_character = test_string.chars().nth((self.max - 1) as usize).unwrap_or_default();

        (first_character == self.character || second_character == self.character) && !(first_character == self.character && second_character == self.character)
    }
}

struct Example {
    password: String,
    policies: Vec<Policy>,
}

impl Example {
    fn new(password: &str) -> Self {
        Example {
            password: password.to_string(),
            policies: vec![],
        }
    }

    fn read_all(path: &str) -> std::io::Result<Vec<Example>> {
        let data_file = std::fs::File::open(path)?;
        let reader = BufReader::new(data_file);

        let mut result: Vec<Example> = vec![];
        let re = regex::Regex::new("(\\d+)-(\\d+)\\s+([a-z])\\s*:\\s*(\\w+)").expect("Should have a legit RE");
        for line in reader.lines() {
            match line {
                Err(error) => return Err(std::io::Error::from(error)),
                Ok(raw_line) => {
                    if raw_line.trim().len() == 0 {
                        return Ok(result);
                    } else {
                        for capture in re.captures_iter(raw_line.trim()) {
                            let min = &capture[1].parse().unwrap_or_default();
                            let max = &capture[2].parse().unwrap_or_default();
                            let character: char = capture[3].chars().next().unwrap();
                            let password = &capture[4];

                            let policy = Policy::new(*min, *max, character);
                            let mut example = Example::new(password);
                            example.add_policy(policy);
                            result.push(example);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn add_policy(&mut self, policy: Policy) {
        self.policies.push(policy)
    }

    fn evaluate(&self) -> bool {
        for policy in self.policies.as_slice() {
            if !policy.evalute(&self.password) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let all_examples = Example::read_all("./input.txt").expect("Should load sample database");
    let mut counter = 0;
    let mut failed = 0;
    for example in all_examples {
        if example.evaluate() {
            counter += 1;
        } else {
            failed += 1;
        }
    }

    println!("There are {} good and {} failed", counter, failed);
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test] 
    fn check_policy_evaluation() {
        let examples = &[
            (1, 3, 'a', "abcde", true),
            (1, 3, 'b', "cdefg", false),
            (2, 9, 'c', "ccccccccc", true)
        ];

        for example in examples {
            let policy = Policy::new(example.0, example.1, example.2);
            let mut test_example = Example::new(example.3);
            test_example.add_policy(policy);

            assert_eq!(test_example.evaluate(), example.4);
        }
    }
}
