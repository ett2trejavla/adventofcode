use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let filename = "input.in";

    let mut input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    input[1] = 12;
    input[2] = 2;
    let output = run_code(&input);
    println!("result: {}", output[0]);

    for i in 0..99 {
        for j in 0..99 {
            input[1] = i;
            input[2] = j;
            let output = run_code(&input);
            if output[0] == 19_690_720 {
                println!("{}{}", i, j);
                return;
            }
        }
    }
}

enum Parameter {
    PositionMode(usize),
    ImmediateMode(i32),
}
enum Operation {
    Add(i32, i32, usize),
    Multiply(i32, i32, usize),
    Input(usize),
    Output(usize),
    Stop,
}

fn parse_op(code: &[i32], pos: &mut usize) -> Operation {}

fn run_code(input: &[i32]) -> Result<Vec<i32>, io::Error> {
    let mut code: Vec<i32> = input.to_vec();
    let mut pos: usize = 0;
    loop {
        let op: Operation = parse_op(&code, &mut pos);
        match op {
            Operation::Add(x, y, write_to) => code[write_to] = x + y,
            Operation::Multiply(x, y, write_to) => code[write_to] = x * y,
            Operation::Input(write_to) => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                code[write_to] = input.trim().parse().unwrap();
            }
            Operation::Output(read_from) => {
                io::stdout().write(code[read_from].to_string().as_bytes());
            }
            Operation::Stop => break,
        };
        pos += 1;
    }
    Ok(code)
}
#[test]
fn test_computer() {
    let input_code = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected_output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    let output = run_code(&input_code).unwrap();
    assert_eq!(output, expected_output);
}
