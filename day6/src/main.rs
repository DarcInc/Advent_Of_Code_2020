use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead,BufReader};

struct Individual {
    answers: HashSet<char>,
}

impl Individual {
    fn new(yeses: &str) -> Self {
        let mut result = Individual {
            answers: HashSet::new(),
        };

        for yes in yeses.chars() {
            result.answers.insert(yes);
        }

        result
    }
}

struct Group {
    individuals: Vec<Box<Individual>>,
    group_answers: HashSet<char>,
    common_answers: HashSet<char>,
}

impl Group {
    fn new() -> Self {
        Group {
            individuals: vec![],
            group_answers: HashSet::new(),
            common_answers: HashSet::new(),
        }
    }

    fn add_individual(&mut self, new_individual: Individual) {
        self.group_answers = self.group_answers.union(&new_individual.answers).cloned().collect();
        if self.individuals.len() == 0 {
            self.common_answers = new_individual.answers.clone();
        } else {
            self.common_answers = self.common_answers.intersection(&new_individual.answers).cloned().collect();
        }
        self.individuals.push(Box::new(new_individual));
    }

    fn total_group_answers(&self) -> i32 {
        self.group_answers.len() as i32
    }

    fn total_common_answers(&self) -> i32 {
        self.common_answers.len() as i32
    }
}

struct Passengers {
    groups: Vec<Box<Group>>,
}

impl Passengers {
    fn new() -> Self {
        Passengers {
            groups: vec![],
        }
    }

    fn read_from_file(path: &str) -> Self {
        let mut result = Passengers::new();

        let input_file = File::open(path).expect("Unable to open input file");
        let reader = BufReader::new(input_file);
        let mut group = Group::new();

        for line in reader.lines() {
            match line {
                Err(error) => panic!("Error reading from file: {}", error),
                Ok(raw_string) => {
                    let trimmed_string = raw_string.trim();
                    if trimmed_string.len() == 0 {
                        result.groups.push(Box::new(group));
                        group = Group::new();
                    } else {
                        let individual = Individual::new(&raw_string);
                        group.add_individual(individual);
                    }
                }
            }
        }

        if !group.individuals.is_empty() {
            result.groups.push(Box::new(group));
        }

        result
    }

    fn sum_group_answers(&self) -> i32 {
        let mut sum = 0;
        for group in &self.groups {
            sum += group.total_group_answers();
        }
        sum as i32
    }

    fn sum_common_answers(&self) -> i32 {
        let mut sum = 0;
        for group in &self.groups {
            sum += group.total_common_answers();
        }
        sum as i32
    }
}

fn main() {
    let passengers = Passengers::read_from_file("./input.txt");

    println!("Sum: {}", passengers.sum_group_answers());

    println!("Common: {}", passengers.sum_common_answers());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_individual() {
        let individual = Individual::new("abc");
        assert!(individual.answers.contains(&'a'));
    }

    #[test]
    fn test_add_individual() {
        let mut g = Group::new();
        let individual_one = Individual::new("ab");
        let individual_two = Individual::new("bc");

        g.add_individual(individual_one);
        g.add_individual(individual_two);

        assert_eq!(2, g.individuals.len());
        assert!(g.group_answers.contains(&'a'));
        assert!(g.group_answers.contains(&'b'));
        assert!(g.group_answers.contains(&'c'));
        assert_eq!(3, g.group_answers.len());
    }

    #[test] 
    fn test_common_answers() {
        let mut g = Group::new();
        let individual_one = Individual::new("abc");
        let individual_two = Individual::new("bcd");

        g.add_individual(individual_one);
        g.add_individual(individual_two);

        assert_eq!(4, g.group_answers.len());
        assert_eq!(2, g.common_answers.len());
    }
}