use std::fs;

enum Op {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}
enum BootResult {
    ReachedEnd(i32),
    InfinitLoop(i32),
}
fn main() {
    let cont = fs::read_to_string("day8.in").expect("Couldn't read file");
    let boot_prog: Vec<Op> = cont.split('\n').map(parse_op).collect();
    let prog_count = 0usize;
    let mut prog_acc = 0i32;
    prog_acc = match boot(&boot_prog, prog_count, prog_acc, false) {
        BootResult::ReachedEnd(acc) => acc,
        BootResult::InfinitLoop(acc) => panic!("inf_loop, acc: {}", acc),
    };
    println!("final acc: {}", prog_acc)
}

fn parse_op(input: &str) -> Op {
    let mut i_iter = input.split_whitespace();
    match i_iter.next() {
        Some("nop") => Op::Nop(i_iter.next().map(|num| num.parse().unwrap()).unwrap()),
        Some("acc") => Op::Acc(i_iter.next().map(|num| num.parse().unwrap()).unwrap()),
        Some("jmp") => Op::Jmp(i_iter.next().map(|num| num.parse().unwrap()).unwrap()),
        _ => panic!("shouldn't happen"),
    }
}

fn boot(program: &[Op], mut pc: usize, mut acc: i32, op_changed: bool) -> BootResult {
    let mut op_count = vec![0; program.len()];

    loop {
        if pc >= program.len() {
            return BootResult::ReachedEnd(acc);
        } else if op_count[pc] != 0 {
            return BootResult::InfinitLoop(acc);
        }
        op_count[pc] += 1;

        if op_changed {
            match program[pc] {
                Op::Acc(arg) => {
                    acc += arg;
                    pc += 1
                }
                Op::Nop(_) => pc += 1,
                Op::Jmp(arg) => pc = (pc as i32 + arg) as usize,
            }
        } else {
            match program[pc] {
                Op::Acc(arg) => {
                    acc += arg;
                    pc += 1
                }
                Op::Nop(arg) => match boot(program,  (pc as i32 + arg) as usize, acc, true) {
                    BootResult::InfinitLoop(_) => pc += 1,
                    BootResult::ReachedEnd(inner_acc) => return BootResult::ReachedEnd(inner_acc),
                },
                Op::Jmp(arg) => match boot(program, pc+1, acc, true) {
                    BootResult::InfinitLoop(_) => pc = (pc as i32 + arg) as usize,
                    BootResult::ReachedEnd(inner_acc) => return BootResult::ReachedEnd(inner_acc),
                },
            }
        }
    }
}
