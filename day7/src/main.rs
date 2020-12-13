use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

struct Clause {
    qty: i32,
    color: String,
}

impl Clause {
    fn new(qty: i32, color: &str) -> Self {
        Clause {
            qty,
            color: String::from(color),
        }
    }

    fn matches(&self, color: &str) -> bool {
        self.color.eq(color)
    }
}

struct Conjunction {
    clauses: Vec<Clause>,
}

impl Conjunction {
    fn parse(text: &str) -> Self {
        static mut COLOR_QTY_EXPR: Option<Regex> = None;

        unsafe {
            if let None = COLOR_QTY_EXPR {
                COLOR_QTY_EXPR = Some(Regex::new(r"(?P<colorqty>(?P<qty>\d+)\s(?P<color>\w+\s\w+)\s(bag|bags)(,)?)+|(?P<none>no other bags)").unwrap());
            }
        }

        let mut result = Conjunction {
            clauses: vec![],
        };

        unsafe {
            if let Some(cqe) = &COLOR_QTY_EXPR {
                for capture in cqe.captures_iter(text) {
                    if let Some(_color_qty) = capture.name("colorqty") {
                        result.clauses.push(Clause::new(capture["qty"].parse().unwrap(), &capture["color"]));
                    }
                }
            }
        }

        result
    }

    fn entails(&self, color: &str) -> bool {
        for clause in &self.clauses {
            if clause.matches(color) {
                return true;
            }
        }
        false
    }
}

struct Rule {
    head: String,
    tail: Conjunction,
}

impl Rule {
    fn parse(text: &str) -> Self {
        static mut HORN_EXPR: Option<Regex> = None;


        unsafe {
            if let None = HORN_EXPR {
                HORN_EXPR = Some(Regex::new(r"(?P<head>\w+\s\w+)\sbags contain\s+(?P<tail>.*)\.").unwrap());
            }
        }

        unsafe {
            if let Some(expression) = &HORN_EXPR {
                let captures = expression.captures(text).unwrap();

                println!("Parsing rule: {}", &captures["head"]);

                Rule {
                    head: String::from(&captures["head"]),
                    tail: Conjunction::parse(&captures["tail"]),
                }
            } else {
                panic!("Missing parse expression");
            }
        }
    }

    fn entails(&self, color: &str) -> bool {
        self.tail.entails(color)
    }
}

struct RuleBase {
    rules: HashMap<String, Rule>,
}

impl RuleBase {
    fn read_from_file(path: &str) -> Self {
        let input_file = File::open(path).expect("File open problem");
        let reader = BufReader::new(input_file);
        let mut result = RuleBase {
            rules: HashMap::new(),
        };

        for line in reader.lines() {
            match line {
                Err(_) => panic!("Failed to read line"),
                Ok(line) => {
                    let new_rule = Rule::parse(&line);
                    result.rules.insert(new_rule.head.clone(), new_rule);
                }
            }
        }

        result
    }

    fn rules_entailing_color(&self, color: &str) -> HashSet<String> {
        let mut result = HashSet::new();

        for rule in &self.rules {
            if rule.1.entails(color) {
                result.insert(rule.0.clone());
            }
        }

        result
    }

    fn all_predecessors(&self, leaf_node: &str) -> HashSet<String> {
        let mut last_size = 0;
        let mut rules_available:HashSet<String> = HashSet::new();
        let mut entailing_rules = self.rules_entailing_color(leaf_node);
        loop {
            for rule_name in &entailing_rules {
                rules_available.insert(rule_name.clone());
            }
    
            if rules_available.len() == last_size {
                break;
            } else {
                entailing_rules.clear();
                for rule_name in &rules_available {
                    let possible_rules = self.rules_entailing_color(rule_name);
                    for some_rule in possible_rules {
                        if !rules_available.contains(&some_rule) {
                            entailing_rules.insert(some_rule.clone());
                        }
                    }
                }
            }
    
            last_size = rules_available.len();
        }
        rules_available
    }

    fn count_all_expanded(&self, rule_name: &str) -> i32 {
        let rule = &self.rules[rule_name];
        let mut result = 0;

        for some_clause in &rule.tail.clauses {
            result += some_clause.qty * self.count_all_expanded(&some_clause.color);
            result += some_clause.qty;
        }


        result
    }
}

fn main() {
    let rule_base = RuleBase::read_from_file("./input.txt");

    let rule = &rule_base.rules.get(&String::from("shiny gold"));
    match rule {
        None => println!("No rule found"),
        Some(rule) => {
            println!("Found rule {}:", &*rule.head);

        }
    }

    let predecessors = rule_base.all_predecessors("shiny gold");
    println!("{} parent rules", predecessors.len());


    let child_bags = rule_base.count_all_expanded("shiny gold"); 
    println!("{} child bags", child_bags);
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn check_regular_expression() {
        let horn_expression = Regex::new("(\\w+\\s\\w+)\\sbags contain\\s+(.*)\\.").unwrap();
        let tail_expression = Regex::new(r"(?P<colorqty>(?P<qty>\d+)\s(?P<color>\w+\s\w+)\s(bag|bags)(,)?)+|(?P<none>no other bags)").unwrap();

        let tests = vec![
            ("light red bags contain 1 bright white bag, 2 muted yellow bags.", "light red", "1 bright white bag, 2 muted yellow bags", 
                vec!["1", "2"], vec!["bright white", "muted yellow"]),
            ("bright white bags contain 1 shiny gold bag.", "bright white", "1 shiny gold bag", vec!["1"], vec!["shiny gold"]),
            ("faded blue bags contain no other bags.", "faded blue", "no other bags", vec![], vec![]),
        ];

        for test in tests {
            let captured = horn_expression.captures(test.0);
            match captured {
                Some(capture) => {
                    assert_eq!(test.1, &capture[1]);
                    assert_eq!(test.2, &capture[2]);

                    let tail_captured = tail_expression.captures_iter(&capture[2]);
                    let mut i = 0;
                    for tail_capture in tail_captured {
                        if let Some(_colorqty) = tail_capture.name("colorqty") {
                            assert!(test.3.len() > 0 && i < test.3.len(), "Should not have matched on color or quantity");

                            assert_eq!(test.3[i], &tail_capture["qty"]);
                            assert_eq!(test.4[i], &tail_capture["color"]);
                        } else if let Some(_no_bags) = tail_capture.name("none") {
                            assert!(test.3.len() == 0, "Should be empty");
                            assert_eq!("no other bags", &tail_capture["none"]);
                        }
                        i += 1;
                    }
                },
                None => { assert!(false, "Failed to capture clause members")},
            }
        }
    }
}
