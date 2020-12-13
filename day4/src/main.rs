use std::io::{Result, BufReader, BufRead};
use std::fs::File;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

fn test_range_number(test: &str, len: usize, min: i32, max: i32) -> bool {
    let number:i32 = test.parse().unwrap_or_default();
    test.trim().len() == len && number >= min && number <= max
}

fn valid_birth_year(test: &str) -> bool {
    test_range_number(test, 4, 1920, 2002)
}

fn valid_issue_year(test: &str) -> bool {
    test_range_number(test, 4, 2010, 2020)
}

fn valid_expiration_year(test: &str) -> bool {
    test_range_number(test, 4, 2020, 2030)
}

fn valid_pid(test: &str) -> bool {
    test_range_number(test, 9, -1, 999999999)
}

fn valid_height(test: &str) -> bool {
    if test.trim().ends_with("cm") {
        let height_cm:i32 = test.trim().trim_end_matches("cm").parse().unwrap_or_default();
        return height_cm >= 150 && height_cm <= 193;
    } else if test.trim().ends_with("in") {
        let height_in:i32 = test.trim().trim_end_matches("in").parse().unwrap_or_default();
        return height_in >= 59 && height_in < 76
    }
    false
}

fn valid_hair_color(test: &str) -> bool {
    let re = Regex::new("#[0-9a-f]{6}").unwrap();
    re.is_match(test)
}

fn valid_eye_color(test: &str) -> bool {
    match test.trim() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

impl Passport {
    fn new() -> Self {
        Passport {
            ecl: String::from(""),
            pid: String::from(""),
            eyr: String::from(""),
            hcl: String::from(""),
            byr: String::from(""),
            iyr: String::from(""),
            cid: String::from(""),
            hgt: String::from(""),
        }
    }

    fn set_value(&mut self, key: &str, value: &str) {
        match key {
            "ecl" => self.ecl = value.to_string(),
            "pid" => self.pid = value.to_string(),
            "eyr" => self.eyr = value.to_string(),
            "hcl" => self.hcl = value.to_string(),
            "byr" => self.byr = value.to_string(),
            "iyr" => self.iyr = value.to_string(),
            "cid" => self.cid = value.to_string(),
            "hgt" => self.hgt = value.to_string(),
            &_ => {}
        }
    }

    fn whats_missing(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        if self.byr.is_empty() { result.push(String::from("byr")) }
        if self.iyr.is_empty() { result.push(String::from("iyr")) }
        if self.eyr.is_empty() { result.push(String::from("eyr")) }
        if self.hgt.is_empty() { result.push(String::from("hgt")) }
        if self.hcl.is_empty() { result.push(String::from("hcl")) }
        if self.ecl.is_empty() { result.push(String::from("ecl")) }
        if self.pid.is_empty() { result.push(String::from("pid")) }
        if self.cid.is_empty() { result.push(String::from("cid")) }

        result
    }

    fn read_passports(path: &str) -> Result<Vec<Passport>> {
        let mut result: Vec<Passport> = vec![];

        let passport_file = File::open(path)?;
        let reader = BufReader::new(passport_file);
        
        let mut p = Passport::new();
        for line in reader.lines() {
            if let Ok(raw_line) = line {
                if raw_line.trim().is_empty() {
                    result.push(p);
                    p = Passport::new();
                } else {
                    let parts_iter = raw_line.split_whitespace();
                    for part in parts_iter {
                        let mut key_value = part.split(':');
                        p.set_value(key_value.next().unwrap_or_default(), key_value.next().unwrap_or_default());
                    }
                }
            } else {
                result.push(p);
                p = Passport::new();
            }
        }
        result.push(p);

        Ok(result)
    }

    fn is_cursory_valid(&self) -> bool {
        !self.byr.is_empty() && !self.iyr.is_empty() && !self.eyr.is_empty() && !self.hgt.is_empty() && !self.hcl.is_empty() && !self.pid.is_empty() && !self.ecl.is_empty()
    }

    fn is_valid(&self) -> bool {
        valid_birth_year(&self.byr) && valid_issue_year(&self.iyr) && valid_expiration_year(&self.eyr) && valid_pid(&self.pid) &&
        valid_height(&self.hgt) && valid_hair_color(&self.hcl) && valid_eye_color(&self.ecl)
    }
}

fn main() {
    let batch = Passport::read_passports("problem.txt").unwrap();
    let mut cursory_valid_count = 0;
    let mut valid_count = 0;
    for passport in batch {
        if passport.is_cursory_valid() {
            cursory_valid_count += 1;
            if passport.is_valid() {
                valid_count += 1;
            }
        } 
    }

    println!("cursory valid count: {}", cursory_valid_count);
    println!("Actual valid: {}", valid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn check_expected_valid() {
        let p = Passport {
            ecl: String::from("gry"),
            pid: String::from("860033327"),
            eyr: String::from("2020"),
            hcl: String::from("#fffffd"),
            byr: String::from("1937"),
            iyr: String::from("2017"),
            cid: String::from("147"),
            hgt: String::from("183cm"),
        };

        assert!(p.is_cursory_valid());

        let p = Passport {
            ecl: String::from("gry"),
            pid: String::from("860033327"),
            eyr: String::from("2020"),
            hcl: String::from("#fffffd"),
            byr: String::from("1937"),
            iyr: String::from("2017"),
            cid: String::from(""),
            hgt: String::from("183cm"),
        };

        assert!(p.is_cursory_valid());
    }
}
