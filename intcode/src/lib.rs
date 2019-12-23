// use std::io::{Read, Write};

#[derive(Debug,PartialEq)]
pub struct Code(Vec<i32>);

struct ProgramState {
    code: Code,
    pos: usize,
    rel_base_pos: i32,
    // prog_in: &'a mut dyn Read,
    // prog_out: &'a mut dyn Write,
}

pub fn run(code: Code) -> Option<Code> {
    let mut state = ProgramState {
        code,
        pos: 0,
        rel_base_pos: 0,
    };
    loop {
        println!("{:?}",&state.code);
        let instruction = parse_instruction(&state)?;
        println!("{:?}",&instruction);
        if let InstructionResult::Break = instruction.process(&mut state) {
            break;
        }
    }
    Some(state.code)
}
trait Instruction {
    const INST_ID: u8;
    const SIZE: usize;
    fn parse_parameters<I>(state: &ProgramState, param_mode: I) -> Option<Self>
    where
        I: Iterator<Item = ParameterMode>,
        Self: Sized;
    fn process(self, state: &mut ProgramState) -> InstructionResult;
}



enum InstructionResult {
    Continue,
    Yield,
    Break,
}

fn parse_instruction(state: &ProgramState) -> Option<Inst> {
    let mut to_parse: String = state.code.0.get(state.pos)?.to_string();
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
     match inst_id {
        Add::INST_ID => Some(Inst::Add(Add::parse_parameters(state, param_mode)?)),
        Multiply::INST_ID=>Some(Inst::Multiply(Multiply::parse_parameters(state, param_mode)?)),
        Stop::INST_ID=>Some(Inst::Stop(Stop::parse_parameters(state, param_mode)?)),
        i => panic!("{} is an unknown instruction", i),
    }
}

fn parse_parameter<'a>(
    state: &'a ProgramState,
) -> impl Fn((&'a i32, ParameterMode, ParameterPriv)) -> Option<i32> {
    move |(val, mode, pri): (&'a i32, ParameterMode, ParameterPriv)| {
        println!("{:?},{:?},{:?}",val,mode,pri);
        let param = match mode {
            ParameterMode::Position => *val,
            ParameterMode::Immediate => return Some(*val),
            ParameterMode::Relative => state.rel_base_pos + *val,
        };

        match pri {
            ParameterPriv::Write => Some(param),
            ParameterPriv::Read => state.code.0.iter().cloned().nth(param as usize),
        }
    }
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
enum Inst{
    Add(Add),
    Multiply(Multiply),
    Stop(Stop)
}
impl Inst{
    fn process(self,state: &mut ProgramState)->InstructionResult{
        match self{
            Inst::Add(instuction)=>instuction.process(state),
            Inst::Multiply(instuction)=>instuction.process(state),
            Inst::Stop(instuction)=>instuction.process(state)
        }
    }
}
#[derive(Debug)]
struct Add(i32, i32, usize);

impl Instruction for Add {
    const INST_ID: u8 = 1;
    const SIZE: usize = 4;
    fn parse_parameters<I>(state: &ProgramState, mut mode: I) -> Option<Self>
    where
        I: Iterator<Item = ParameterMode>,
    {
        let parser = parse_parameter(state);
        let mut cur_code_pos = state.code.0.iter().skip(state.pos+1);

        let term1 = parser((cur_code_pos.next()?, mode.next()?, ParameterPriv::Read))?;
        let term2 = parser((cur_code_pos.next()?, mode.next()?, ParameterPriv::Read))?;
        let write_sum_to = parser((cur_code_pos.next()?, mode.next()?, ParameterPriv::Write))?;

        Some(Self(term1, term2, write_sum_to as usize))
    }
    fn process(self, state: &mut ProgramState) -> InstructionResult {
        let Self(term1, term2, write_sum_to) = self;

        *state.code.0.get_mut(write_sum_to).unwrap() = term1 + term2;
        
        state.pos += Self::SIZE;
        InstructionResult::Continue
    }
}

#[derive(Debug)]
struct Multiply(i32, i32, usize);

impl Instruction for Multiply {
    const INST_ID: u8 = 2;
    const SIZE: usize = 4;
    fn parse_parameters<I>(state: &ProgramState, mut mode: I) -> Option<Self>
    where
        I: Iterator<Item = ParameterMode>,
    {
        let parser = parse_parameter(state);
        let mut cur_code_pos = state.code.0.iter().skip(state.pos+1);

        let factor1 = parser((cur_code_pos.next()?, mode.next()?, ParameterPriv::Read))?;
        let factor2 = parser((cur_code_pos.next()?, mode.next()?, ParameterPriv::Read))?;
        let write_prod_to = parser((cur_code_pos.next()?, mode.next()?, ParameterPriv::Write))?;

        Some(Self(factor1, factor2, write_prod_to as usize))
    }
    fn process(self, state: &mut ProgramState) -> InstructionResult {
        let Self(factor1, factor2, write_sum_to) = self;

        *state.code.0.get_mut(write_sum_to).unwrap() = factor1 * factor2;

        state.pos += Self::SIZE;
        InstructionResult::Continue
    }
}

#[derive(Debug)]
struct Stop;

impl Instruction for Stop {
    const INST_ID: u8 = 99;
    const SIZE: usize = 1;
    fn parse_parameters<I>(_: &ProgramState, _: I) -> Option<Self>
    where
        I: Iterator<Item = ParameterMode>,
    {
        Some(Self)
    }
    fn process(self, _: &mut ProgramState) -> InstructionResult {
        InstructionResult::Break
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_computer() {
        let input_code = Code(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        let expected_output = Code(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        let output = run(input_code).unwrap();
        assert_eq!(output, expected_output);
    }
}
