use std::io::{Read, Write};

pub struct ProgramState {
    code: Vec<i64>,
    pos: usize,
    rel_base_pos: i64,
}
pub struct PausedState<'prog_run> {
    state: ProgramState,
    prog_in: &'prog_run mut dyn Read,
    prog_out: &'prog_run mut dyn Write,
    inst: Inst,
}

pub fn start_run<'prog_run>(
    code: Vec<i64>,
    prog_in: &'prog_run mut dyn Read,
    prog_out: &'prog_run mut dyn Write,
) -> Option<Vec<i64>> {
    let state = ProgramState {
        code,
        pos: 0,
        rel_base_pos: 0,
    };
    run(state, prog_in, prog_out)
}
pub fn resume_run(mut ps: PausedState) -> Option<Vec<i64>> {
    if let InstructionResult::Stop = ps.inst.process(&mut ps.state, ps.prog_in, ps.prog_out) {
        None
    } else {
        run(ps.state, ps.prog_in, ps.prog_out)
    }
}
fn run<'prog_run>(
    mut state: ProgramState,
    prog_in: &'prog_run mut dyn Read,
    prog_out: &'prog_run mut dyn Write,
) -> Option<Vec<i64>> {
    loop {
        println!("{:?}", &state.code);
        let instruction = parse_instruction(&state)?;
        println!("{:?}", &instruction);
        if let InstructionResult::Stop = instruction.process(&mut state, prog_in, prog_out) {
            break;
        }
    }
    Some(state.code)
}

enum InstructionResult {
    Continue,
    Yield,
    Stop,
}

fn parse_instruction<'a>(state: &'a ProgramState) -> Option<Inst> {
    let mut code_iter = state.code.iter().skip(state.pos).take(Inst::MAX_SIZE);
    let mut to_parse: String = code_iter.next()?.to_string();
    while to_parse.len() < 5 {
        to_parse.insert(0, '0');
    }
    assert_eq!(to_parse.len(), 5);
    let inst_id = to_parse[3..].parse::<u8>().unwrap();
    let param_mode = to_parse[0..3].chars().rev().map(|pm| -> ParameterMode {
        match pm {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            '2' => ParameterMode::Relative,
            c => panic!(
                "failed to parse parameter modes, {} is not a valid parameter mode",
                c
            ),
        }
    });
    Inst::new(inst_id, code_iter, param_mode, state)
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}
#[derive(Debug)]
enum ParameterPriv {
    Read,
    Write,
}
#[derive(Debug)]
pub enum Inst {
    Add(i64, i64, usize),
    Multiply(i64, i64, usize),
    Input(usize),
    Output(i64),
    JumpIfTrue(i64, usize),
    JumpIfFalse(i64, usize),
    LessThan(i64, i64, usize),
    Equals(i64, i64, usize),
    RelativeBaseOffset(i64),
    Stop,
}
impl Inst {
    const MAX_SIZE: usize = 4;
    fn new<'a, I, J>(
        inst_id: u8,
        mut code_iter: I,
        mut mode: J,
        state: &'a ProgramState,
    ) -> Option<Self>
    where
        I: Iterator<Item = &'a i64>,
        J: Iterator<Item = ParameterMode>,
    {
        let parser = parameter_parser(state);
        match inst_id {
            1 => Some(Inst::Add(
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Write))? as usize,
            )),
            2 => Some(Inst::Multiply(
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Write))? as usize,
            )),
            3 => Some(Inst::Input(
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Write))? as usize,
            )),
            4 => Some(Inst::Output(parser((
                code_iter.next()?,
                mode.next()?,
                ParameterPriv::Read,
            ))?)),
            5 => Some(Inst::JumpIfTrue(
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))? as usize,
            )),
            6 => Some(Inst::JumpIfFalse(
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))? as usize,
            )),
            7 => Some(Inst::LessThan(
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Write))? as usize,
            )),
            8 => Some(Inst::Equals(
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Read))?,
                parser((code_iter.next()?, mode.next()?, ParameterPriv::Write))? as usize,
            )),
            9 => Some(Inst::RelativeBaseOffset(parser((
                code_iter.next()?,
                mode.next()?,
                ParameterPriv::Read,
            ))?)),
            99 => Some(Inst::Stop),
            i => panic!("{} is not a valid instruction_id", i),
        }
    }
    fn size(&self) -> usize {
        match *self {
            Inst::Add(_, _, _)
            | Inst::Multiply(_, _, _)
            | Inst::LessThan(_, _, _)
            | Inst::Equals(_, _, _) => 4,
            Inst::JumpIfTrue(_, _) | Inst::JumpIfFalse(_, _) => 3,
            Inst::Input(_) | Inst::Output(_) | Inst::RelativeBaseOffset(_) => 2,
            Inst::Stop => 1,
        }
    }
    fn process(
        self,
        state: &mut ProgramState,
        mut prog_in: &mut dyn Read,
        prog_out: &mut dyn Write,
    ) -> InstructionResult {
        let res = match self {
            Inst::Add(term1, term2, write_sum_to) => {
                *state.code.get_mut(write_sum_to).unwrap() = term1 + term2;
                state.pos += self.size();

                InstructionResult::Continue
            }
            Inst::Multiply(factor1, factor2, write_prod_to) => {
                *state.code.get_mut(write_prod_to).unwrap() = factor1 * factor2;
                state.pos += self.size();

                InstructionResult::Continue
            }
            Inst::Input(write_to) => {
                use std::io::{BufRead, BufReader};
                let mut input = String::new();
                let mut locked_input = BufReader::new(&mut prog_in);
                locked_input.read_line(&mut input).unwrap();
                state.code[write_to] = input.trim().parse().unwrap();
                state.pos += self.size();

                InstructionResult::Continue
            }
            Inst::Output(read_from) => {
                prog_out.write_fmt(format_args!("{}\n", read_from)).unwrap();
                state.pos += self.size();

                InstructionResult::Continue
            }
            Inst::JumpIfTrue(val, jmp_to) => {
                state.pos = if val != 0 {
                    jmp_to
                } else {
                    state.pos + self.size()
                };
                InstructionResult::Continue
            }
            Inst::JumpIfFalse(val, jmp_to) => {
                state.pos = if val == 0 {
                    jmp_to
                } else {
                    state.pos + self.size()
                };
                InstructionResult::Continue
            }
            Inst::LessThan(a, b, write_to) => {
                state.code[write_to] = if a < b { 1 } else { 0 };
                state.pos += self.size();

                InstructionResult::Continue
            }
            Inst::Equals(a, b, write_to) => {
                state.code[write_to] = if a == b { 1 } else { 0 };
                state.pos += self.size();

                InstructionResult::Continue
            }
            Inst::RelativeBaseOffset(offset) => {
                state.rel_base_pos += offset;
                state.pos += self.size();
                InstructionResult::Continue
            }
            Inst::Stop => InstructionResult::Stop,
        };
        res
    }
}

fn parameter_parser<'a>(
    state: &'a ProgramState,
) -> impl Fn((&'a i64, ParameterMode, ParameterPriv)) -> Option<i64> {
    move |(val, mode, pri): (&'a i64, ParameterMode, ParameterPriv)| {
        println!("{:?},{:?},{:?}", val, mode, pri);
        let param = match mode {
            ParameterMode::Position => *val,
            ParameterMode::Immediate => return Some(*val),
            ParameterMode::Relative => state.rel_base_pos + *val,
        };

        match pri {
            ParameterPriv::Write => Some(param),
            ParameterPriv::Read => state.code.iter().cloned().nth(param as usize),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_mul() {
        use std::io::{stdin, stdout};
        let input_code = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected_output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut prog_in = stdin();
        let output = start_run(input_code, &mut prog_in, &mut stdout()).unwrap();
        assert_eq!(output, expected_output);
    }
    use std::io::BufRead;
    use std::io::Cursor;
    #[test]
    fn test_jump_pos() {
        let mut prog_in = Cursor::new(vec![b'0', b'\n']);
        let mut prog_out = Cursor::new(Vec::<u8>::new());
        let input_code = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        start_run(input_code.clone(), &mut prog_in, &mut prog_out).unwrap();
        prog_out.set_position(0);
        let mut out = String::new();
        prog_out.read_line(&mut out).unwrap();
        assert_eq!(out.trim().parse::<u8>().unwrap(), 0u8);

        prog_in = Cursor::new(vec![b'1', b'\n']);
        prog_out = Cursor::new(Vec::<u8>::new());
        start_run(input_code, &mut prog_in, &mut prog_out).unwrap();
        prog_out.set_position(0);
        let mut out = String::new();
        prog_out.read_line(&mut out).unwrap();
        assert_eq!(out.trim().parse::<u8>().unwrap(), 1u8);
    }
    #[test]
    fn test_immediate_mode() {
        let mut prog_in = Cursor::new(vec![b'0', b'\n']);
        let mut prog_out = Cursor::new(Vec::<u8>::new());
        let input_code = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        start_run(input_code.clone(), &mut prog_in, &mut prog_out).unwrap();
        prog_out.set_position(0);
        let mut out = String::new();
        prog_out.read_line(&mut out).unwrap();
        assert_eq!(out.trim().parse::<u8>().unwrap(), 0u8);

        prog_in = Cursor::new(vec![b'1', b'\n']);
        prog_out = Cursor::new(Vec::<u8>::new());
        start_run(input_code, &mut prog_in, &mut prog_out).unwrap();
        prog_out.set_position(0);
        let mut out = String::new();
        prog_out.read_line(&mut out).unwrap();
        assert_eq!(out.trim().parse::<u8>().unwrap(), 1u8);
    }
}
