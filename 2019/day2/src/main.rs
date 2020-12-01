use std::fs;

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
fn run_code(input: &[u32]) -> Vec<u32> {
    let mut code: Vec<u32> = input.to_vec();
    let mut j: usize = 0;
    loop {
        let instruction: Vec<u32> = code.iter().skip(4 * j).take(4).cloned().collect();
        match instruction.first() {
            Some(1) => {
                code[instruction[3] as usize] =
                    code[instruction[1] as usize] + code[instruction[2] as usize]
            }
            Some(2) => {
                code[instruction[3] as usize] =
                    code[instruction[1] as usize] * code[instruction[2] as usize]
            }
            Some(99) => break,
            Some(i) => panic!("instruction {}", i),
            None => panic!("No value"),
        };
        j += 1;
    }
    code
}
#[test]
fn test_computer() {
    let input_code = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected_output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    let output = run_code(&input_code);
    assert_eq!(output, expected_output);
}
