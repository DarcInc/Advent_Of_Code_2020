use regex::Regex;
use std::fmt;
use std::convert::From;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

#[derive(Debug)]
enum ParseError {
    Error,
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse operation")
    }
}

impl From<std::fmt::Error> for ParseError {
    fn from(error: std::fmt::Error) -> Self {
        ParseError::Error
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParseError::Error
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Operations {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Operations::Nop(amt) => {
                f.write_fmt(format_args!("nop {}", amt))
            },
            Operations::Jmp(amt) => {
                f.write_fmt(format_args!("jmp {}", amt))
            },
            Operations::Acc(amt) => {
                f.write_fmt(format_args!("acc {}", amt))
            }
        }
    }
}

impl Operations {
    fn new(op: &str, sign: &str, qty: &str) -> Result<Self, ParseError> {
        let mut qty:i32 = qty.parse()?;
        if sign == "-" {
            qty = -qty;
        }

        match op {
            "acc" => Ok(Operations::Acc(qty)),
            "jmp" => Ok(Operations::Jmp(qty)),
            "nop" => Ok(Operations::Nop(qty)),
            _ => Err(ParseError::Error),
        }
    }

    fn flip(&self) -> Operations {
        match self {
            Operations::Jmp(amt) => Operations::Nop(*amt),
            Operations::Nop(amt) => Operations::Jmp(*amt),
            Operations::Acc(amt) => Operations::Acc(*amt),
        }
    }
}

#[derive(Debug)]
enum AssemblyError {
    Failed(usize),
}

impl From<ParseError> for AssemblyError {
    fn from(error: ParseError) -> Self {
        AssemblyError::Failed(0)
    }
}

struct Assembler {
    op_codes: Vec<Operations>,
    parse_expression: regex::Regex,
}

impl Assembler {
    fn new() -> Self {
        Assembler {
            op_codes: vec![],
            parse_expression: Regex::new(r"(?P<op>(jmp|nop|acc))\s+(?P<sign>[\-+])(?P<qty>\d+)").unwrap(),
        }
    }

    fn parse_file(path: &str) -> Self {
        let input_file = File::open(path).expect("Unable to open input file");
        let reader = BufReader::new(input_file);
        let mut assembler = Assembler::new();
        let mut line_number = 0;
    
        for line in reader.lines() {
            match line {
                Ok(program_text) => {
                    match assembler.assemble(&program_text) {
                        Ok(_) => {},
                        Err(_) => {
                            println!("Error on line {}: {}", line_number, program_text);
                        }
                    }
                },
                Err(_) => {
                    println!("Line number: {}", line_number);
                    panic!("Failed to read program");
                }
            }
            line_number += 1;
        }
        
        assembler
    }

    fn assemble(&mut self, line: &str) -> Result<(), AssemblyError> {
        let captures = self.parse_expression.captures(line);
        match captures {
            None => return Err(AssemblyError::Failed(self.op_codes.len())),
            Some(captured) => {
                let new_op = Operations::new(&captured["op"], &captured["sign"], &captured["qty"])?;
                self.op_codes.push(new_op);
            }
        }
        Ok(())
    }

    fn program(&self) -> Vec<Operations> {
        let mut result = vec![];

        for op in &self.op_codes {
            result.push(op.clone());
        }

        result
    }
}

struct Processor {
    instruction_pointer: u64,
    accumulator: i64,
    op_codes: Vec<Operations>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            instruction_pointer: 0,
            accumulator: 0,
            op_codes: vec![],
        }
    }

    fn load(&mut self, program: &Vec<Operations>) {
        for op in program {
            self.op_codes.push(op.clone());
        }
    }

    fn step(&mut self) {
        let op = &self.op_codes[self.instruction_pointer as usize];
        match op {
            Operations::Nop(_) => {
                self.instruction_pointer += 1;
            },
            Operations::Acc(amt) => {
                self.accumulator += *amt as i64;
                self.instruction_pointer += 1;
            }
            Operations::Jmp(amt) => {
                if *amt > 0 {
                    self.instruction_pointer += *amt as u64;
                } else {
                    self.instruction_pointer -= -amt as u64;
                }
            }
        }
    }

    fn trace(&mut self) -> Option<InstructionTrace> {
        let old_ip = self.instruction_pointer;
        let old_acc = self.accumulator;
        if old_ip >= self.op_codes.len() as u64 {
            return None;
        } 

        let op = &self.op_codes[self.instruction_pointer as usize].clone();

        self.step();

        Some(InstructionTrace::new(old_ip, old_acc, &op, self.instruction_pointer, self.accumulator))
    }

    fn accumulator(&self) -> i64 {
        self.accumulator
    }

    fn instruction_pointer(&self) -> u64 {
        self.instruction_pointer
    }
}

#[derive(Clone)]
struct InstructionTrace {
    instruction_pointer: u64,
    new_instruction_pointer: u64,
    accumulator: i64,
    new_accumulator: i64,
    instruction: Operations,
}

impl InstructionTrace {
    fn new(ip: u64, acc: i64, op: &Operations, new_ip: u64, new_acc:i64) -> Self {
        InstructionTrace {
            instruction_pointer: ip,
            new_instruction_pointer: new_ip,
            accumulator: acc,
            new_accumulator: new_acc,
            instruction: op.clone(),
        }
    }
}

impl std::fmt::Display for InstructionTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("ip:{} op:{} acc:{} -> ip:{} acc{}", 
            self.instruction_pointer, self.instruction, self.accumulator, 
            self.new_instruction_pointer, self.new_accumulator))
    }
}

enum TerminationCondition {
    InfiniteLoop,
    Normal,
}

struct Trace {
    history: Vec<InstructionTrace>,
}

impl Trace {
    fn new() -> Self {
        Trace {
            history: vec![],
        }
    }

    fn append(&mut self, itrace: &InstructionTrace) {
        self.history.push(itrace.clone())
    }

    fn run_trace(&mut self, processor: &mut Processor) -> TerminationCondition {
        let mut visited_lines:HashSet<u64> = HashSet::new();
   
        loop {
            let itrace = processor.trace();
            match itrace {
                None => {
                    return TerminationCondition::Normal;
                },
                Some(itrace) => {
                    visited_lines.insert(itrace.instruction_pointer);

                    if visited_lines.contains(&itrace.new_instruction_pointer) {
                        return TerminationCondition::InfiniteLoop;
                    }

                    self.append(&itrace);
                }
            }
             
        }
    }
}


fn main() {
    let assembler = Assembler::parse_file("./input.txt");

    let mut processor = Processor::new();
    let program = assembler.program();

    processor.load(&program);
    
    let mut trace = Trace::new();

    match trace.run_trace(&mut processor) {
        TerminationCondition::InfiniteLoop => {
            match trace.history.last() {
                None => println!("Failed to get a trace"),
                Some(trace) => {
                    println!("{}", trace.accumulator);
                },
            }
        },
        TerminationCondition::Normal => {
            println!("Failed to get an infinite loop");
        }
    }

    let mut program = assembler.program();
    let mut has_mutated = false;
    let mut mutated_ip:usize = 0;
    'outer: loop {
        let mut trace = Trace::new();
        
        let mut processor = Processor::new();
        processor.load(&program);

        match trace.run_trace(&mut processor) {
            TerminationCondition::InfiniteLoop => {
                if has_mutated {
                    program[mutated_ip] = program[mutated_ip].flip();
                    mutated_ip += 1;
                }

                loop {
                    match program[mutated_ip] {
                        Operations::Acc(_) => mutated_ip += 1,
                        _ => break,
                    }
                }

                println!("\tMutated IP: {}", mutated_ip);
                program[mutated_ip] = program[mutated_ip].flip();
                has_mutated = true;
            },
            TerminationCondition::Normal => {
                let last_trace = trace.history.last().unwrap();
                println!("Accumulatorx: {}", last_trace.accumulator);
                break 'outer;
            }
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_patterns() {
        let test_data = &[
            ("nop +1", Operations::Nop(1)),
            ("jmp -4", Operations::Jmp(-4)),
            ("acc +4", Operations::Acc(4)),
        ];

        let pattern = r"(?P<op>(jmp|nop|acc))\s+(?P<sign>[\-+])(?P<qty>\d+)";
        let re = Regex::new(pattern).unwrap();

        for test_datum in test_data {
            let captures = re.captures(test_datum.0).unwrap();
            match Operations::new(&captures["op"], &captures["sign"], &captures["qty"]) {
                Err(_) => assert!(false, "Failed to parse operation"),
                Ok(machine_op) => assert_eq!(test_datum.1, machine_op),
            }          
        }
    }

    #[test]
    fn test_assembler() {
        let test_data = &[
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ];

        let expected_data = &[
            Operations::Nop(0),
            Operations::Acc(1),
            Operations::Jmp(4),
            Operations::Acc(3),
            Operations::Jmp(-3),
            Operations::Acc(-99),
            Operations::Acc(1),
            Operations::Jmp(-4),
            Operations::Acc(6),
        ];

        let mut assembler = Assembler::new();
        for line in test_data {
            assembler.assemble(line).expect("Failed to parse line");
        }

        let pgm = assembler.program();
        assert_eq!(expected_data.len(), pgm.len());
        for i in 0..pgm.len() {
            assert_eq!(expected_data[i], pgm[i]);
        }
    }
}