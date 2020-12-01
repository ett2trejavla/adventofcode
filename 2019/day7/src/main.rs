use permutohedron::LexicalPermutation;
use std::cmp;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Cursor, Read, Write};

fn main() {
    let filename = "input.in";

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let output = phase_permuations(&input, &mut [0, 1, 2, 3, 4]);
    println!("reached end: {}", output);
}


struct ProgramState {
    code: Vec<i32>,
    pos: usize,
    rel_pos_base: usize,
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
impl Operation{
    fn parse()->Operation{
    }
}

fn parse_op(code: &[i32], pos: &mut usize) -> Option<Operation> {
    let op: String = {
        let mut op: String = code.get(*pos)?.to_string();
        while op.len() < 5 {
            op.insert(0, '0');
        }
        op
    };

    let op_code: u32 = op[3..].parse().unwrap();

    let mut param_mode = op.chars().rev().skip(2);
    let mut code_val = code.iter().skip(*pos + 1).take(3).cloned();
    let mut parse_param = || match param_mode.next() {
        Some('1') => code_val.next().unwrap(),
        Some('0') | None => code[code_val.next().unwrap() as usize],
        Some(c) => panic!("Not a parameter code :{}", c),
    };

    match op_code {
        1 => Some(Operation::Add(
            parse_param(),
            parse_param(),
            code_val.next().unwrap() as usize,
        )),
        2 => Some(Operation::Multiply(
            parse_param(),
            parse_param(),
            code_val.next().unwrap() as usize,
        )),
        3 => Some(Operation::Input(code_val.next().unwrap() as usize)),
        4 => Some(Operation::Output(parse_param() as usize)),
        5 => Some(Operation::JumpIfTrue(parse_param(), parse_param() as usize)),
        6 => Some(Operation::JumpIfFalse(
            parse_param(),
            parse_param() as usize,
        )),
        7 => Some(Operation::LessThen(
            parse_param(),
            parse_param(),
            code_val.next().unwrap() as usize,
        )),
        8 => Some(Operation::Equals(
            parse_param(),
            parse_param(),
            code_val.next().unwrap() as usize,
        )),
        99 => Some(Operation::Stop),
        i => panic!("Not a valid operation: {}", i),
    }
}
fn run_code(
    input: &[i32],
    mut prog_input: &mut dyn Read,
    prog_output: &mut dyn Write,
) -> Option<Vec<i32>> {
    let mut p_state = ProgramState {
        code:input.to_vec(),
        pos: 0,
        rel_pos_base: 0,
    };
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
                let mut locked_input = BufReader::new(&mut prog_input);
                locked_input.read_line(&mut input).unwrap();
                code[write_to] = input.trim().parse().unwrap();
                pos += 2;
            }
            Operation::Output(read_from) => {
                prog_output
                    .write_fmt(format_args!("{}\n", read_from))
                    .unwrap();
                pos += 2;
            }
            Operation::JumpIfTrue(b, n_pos) => pos = if b != 0 { n_pos } else { pos + 3 },
            Operation::JumpIfFalse(b, n_pos) => pos = if b == 0 { n_pos } else { pos + 3 },
            Operation::LessThen(a, b, write_to) => {
                code[write_to] = if a < b { 1 } else { 0 };
                pos += 4;
            }
            Operation::Equals(a, b, write_to) => {
                code[write_to] = if a == b { 1 } else { 0 };
                pos += 4;
            }
            Operation::Stop => break,
        };
    }
    Some(code)
}
fn phase_permuations(code: &[i32], phase_list: &mut [i32; 5]) -> i32 {
    let mut max_thrust = std::i32::MIN;
    max_thrust = cmp::max(amp_controller(code, *phase_list), max_thrust);
    while phase_list.next_permutation() {
        max_thrust = cmp::max(amp_controller(code, *phase_list), max_thrust);
    }
    max_thrust
}

fn amp_controller(code: &[i32], setting_seq: [i32; 5]) -> i32 {
    let mut input = Cursor::new(Vec::new());
    let mut out = "0".to_string();
    let mut output = Cursor::new(Vec::new());
    for phase in setting_seq.iter() {
        input
            .write_fmt(format_args!("{}\n{}\n", phase, out))
            .unwrap();
        input.set_position(0);
        output.set_position(0);
        run_code(&code, &mut input, &mut output).unwrap();
        input.set_position(0);
        output.set_position(0);
        out = "".to_string();
        output.read_line(&mut out).unwrap();
    }
    out.trim().parse().unwrap()
}

#[test]
fn test_computer() {
    let input_code = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected_output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    let output = run_code(&input_code, &mut io::stdin().lock(), &mut io::stdout()).unwrap();
    assert_eq!(output, expected_output);
}
#[test]
fn test_comparator_instructions() {
    use std::io::{BufRead, Cursor};
    let input_code = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    {
        let mut input = Cursor::new("5\n".as_bytes());
        let mut output = Cursor::new(Vec::new());

        run_code(&input_code, &mut input, &mut output).unwrap();
        output.set_position(0);
        let mut out = String::new();
        output.read_line(&mut out).unwrap();
        assert_eq!(out.trim_end().parse::<i32>().unwrap(), 1);
    }

    {
        let mut input = Cursor::new("0\n".as_bytes());
        let mut output = Cursor::new(Vec::new());

        run_code(&input_code, &mut input, &mut output).unwrap();
        output.set_position(0);
        let mut out = String::new();
        output.read_line(&mut out).unwrap();
        assert_eq!(out.trim_end().parse::<i32>().unwrap(), 0);
    }

    let large_input = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    {
        let mut input = Cursor::new("5\n".as_bytes());
        let mut output = Cursor::new(Vec::new());

        run_code(&large_input, &mut input, &mut output).unwrap();
        output.set_position(0);
        let mut out = String::new();
        output.read_line(&mut out).unwrap();
        assert_eq!(out.trim_end().parse::<i32>().unwrap(), 999);
    }
    {
        let mut input = Cursor::new("8\n".as_bytes());
        let mut output = Cursor::new(Vec::new());

        run_code(&large_input, &mut input, &mut output).unwrap();
        output.set_position(0);
        let mut out = String::new();
        output.read_line(&mut out).unwrap();
        assert_eq!(out.trim_end().parse::<i32>().unwrap(), 1000);
    }
    {
        let mut input = Cursor::new("12\n".as_bytes());
        let mut output = Cursor::new(Vec::new());

        run_code(&large_input, &mut input, &mut output).unwrap();
        output.set_position(0);
        let mut out = String::new();
        output.read_line(&mut out).unwrap();
        assert_eq!(out.trim_end().parse::<i32>().unwrap(), 1001);
    }
}
#[test]
fn amp_small() {
    let code = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    assert_eq!(amp_controller(&code, [4, 3, 2, 1, 0]), 43210)
}

#[test]
fn amp_permutations_small() {
    {
        let code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(phase_permuations(&code, &mut [0, 1, 2, 3, 4]), 43210)
    }
    {
        let code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(phase_permuations(&code, &mut [0, 1, 2, 3, 4]), 54321)
    }
    {
        let code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(phase_permuations(&code, &mut [0, 1, 2, 3, 4]), 65210)
    }
}
