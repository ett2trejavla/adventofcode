use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let filename = "input.in";

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let output = run_code(&input).unwrap();
    println!("reached end: {}", output[0]);
}
#[derive(Debug)]
enum Operation {
    Add(i32, i32, usize),
    Multiply(i32, i32, usize),
    Input(usize),
    Output(usize),
    JumpIfTrue(i32, usize),
    JumpIfFalse(i32, usize),
    LessThen(i32, i32, usize),
    Equals(i32, i32, usize),
    Stop,
}

fn parse_op(code: &[i32], pos: &mut usize) -> Option<Operation> {
    let op: String = code.iter().nth(*pos)?.to_string();

    let op_code: u32 = if op.len() == 1 {
        op.parse().unwrap()
    } else {
        op[(op.len() - 2)..].parse().unwrap()
    };

    let mut param_mode = op.chars().rev().skip(2);
    let mut code_val = code.iter().skip(*pos + 1).take(3).cloned();
    let mut parse_param = || match param_mode.next() {
        Some('1') => code_val.next().unwrap(),
        Some('0') | None => code[code_val.next().unwrap() as usize],
        Some(c) => panic!("Not a parameter code :{}", c),
    };

    match op_code {
        1 => {
            let p1 = parse_param();
            let p2 = parse_param();

            let write_to: usize = code_val.next().unwrap() as usize;

            return Some(Operation::Add(p1, p2, write_to));
        }
        2 => {
            let p1 = parse_param();
            let p2 = parse_param();

            let write_to: usize = code_val.next().unwrap() as usize;

            return Some(Operation::Multiply(p1, p2, write_to));
        }
        3 => {
            let mut code_v = code.iter().skip(*pos + 1).take(1).cloned();

            let write_to: usize = code_v.next().unwrap() as usize;

            Some(Operation::Input(write_to))
        }
        4 => {

            let read_from: usize = parse_param() as usize;

            Some(Operation::Output(read_from))
        }
        5 => {
            let p1 = parse_param();

            let write_to: usize = parse_param() as usize;

            return Some(Operation::JumpIfTrue(p1, write_to));
        }
        6 => {
            let p1 = parse_param();

            let write_to: usize = parse_param() as usize;

            return Some(Operation::JumpIfFalse(p1, write_to));
        }
        7 => {
            let p1 = parse_param();
            let p2 = parse_param();

            let write_to: usize = code_val.next().unwrap() as usize;

            return Some(Operation::LessThen(p1, p2, write_to));
        }
        8 => {
            let p1 = parse_param();
            let p2 = parse_param();

            let write_to: usize = code_val.next().unwrap() as usize;

            return Some(Operation::Equals(p1, p2, write_to));
        }
        99 => return Some(Operation::Stop),
        i => panic!("Not a valid operation: {}", i),
    }
}

fn run_code(input: &[i32]) -> Option<Vec<i32>> {
    let mut code: Vec<i32> = input.to_vec();
    let mut pos: usize = 0;
    loop {
        let op: Operation = parse_op(&code, &mut pos)?;
        println!("{:?}  {}", op, pos);
        match op {
            Operation::Add(x, y, write_to) => {
                code[write_to] = x + y;
                pos += 4;
            }
            Operation::Multiply(x, y, write_to) => {
                code[write_to] = x * y;
                pos += 4;
            }
            Operation::Input(write_to) => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                code[write_to] = input.trim().parse().unwrap();
                pos += 2;
            }
            Operation::Output(read_from) => {
                io::stdout()
                    .write_fmt(format_args!("{}\n", read_from))
                    .unwrap();
                pos += 2;
            }
            Operation::JumpIfTrue(b, n_pos) => pos = if b != 0 { n_pos } else { pos + 3 },
            Operation::JumpIfFalse(b, n_pos) => pos = if b == 0 { n_pos } else { pos + 3 },
            Operation::LessThen(a, b, write_to) => {
                code[write_to] = if a < b { 1 } else { 0 };
                pos += 4;
            },
            Operation::Equals(a, b, write_to) => {
                code[write_to] = if a == b { 1 } else { 0 };
                pos += 4;
            },
            Operation::Stop => break,
        };
    }
    Some(code)
}
#[test]
fn test_computer() {
    let input_code = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected_output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    let output = run_code(&input_code).unwrap();
    assert_eq!(output, expected_output);
}
#[test]
fn test_comparator_instructions() {
    let input_code = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    run_code(&input_code).unwrap();

    run_code(&input_code).unwrap();

    let large_input = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    run_code(&large_input).unwrap();
    run_code(&large_input).unwrap();
}
