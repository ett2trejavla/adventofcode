use std::error;
use std::fmt;
use std::fs;
use std::str::FromStr;
#[derive(Debug, PartialEq)]
struct PassHeuristic {
    lower_bound: usize,
    upper_bound: usize,
    c: char,
    password: String,
}
#[derive(Debug)]
struct ParseErrorCharToFind;

impl error::Error for ParseErrorCharToFind {}

impl fmt::Display for ParseErrorCharToFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Couldn't parse the char to search for")
    }
}

impl FromStr for PassHeuristic {
    type Err = Box<dyn error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let bounds: Vec<&str> = parts[0].split("-").collect();
        let c = parts[1].chars().next().ok_or(ParseErrorCharToFind)?;

        Ok(PassHeuristic {
            lower_bound: bounds[0].parse()?,
            upper_bound: bounds[1].parse()?,
            c,
            password: parts[2].to_string(),
        })
    }
}

fn main() {
    let cont = fs::read_to_string("day2.in").expect("Couldn't read file");
    let pass_heuristics: Vec<PassHeuristic> = cont
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    solve_1(&pass_heuristics);
    solve_2(&pass_heuristics)
}

fn solve_1(lines: &[PassHeuristic]) {
    let valid_passwords = lines.iter().filter(valid_password).count();
    println!(
        "Found there to be {} number of valid passwords",
        valid_passwords
    )
}

fn valid_password(heur: &&PassHeuristic) -> bool {
    let PassHeuristic {
        lower_bound: lb,
        upper_bound: ub,
        c,
        password: pass,
    } = heur;

    let num_line_char = pass.chars().filter(|x| *x == *c).count();
    *lb <= num_line_char && num_line_char <= *ub
}

fn solve_2(lines: &[PassHeuristic]) {
    let valid_passwords = lines.iter().filter(valid_password2).count();
    println!(
        "Found there to be {} number of valid passwords",
        valid_passwords
    )
}

fn valid_password2(heur: &&PassHeuristic) -> bool {
    let PassHeuristic {
        lower_bound: lb,
        upper_bound: ub,
        c,
        password: pass,
    } = heur;

    let fst_char = pass.chars().nth(lb - 1);
    let snd_char = pass.chars().nth(ub - 1);
    match (fst_char, snd_char) {
        (Some(c1), Some(c2)) => (*c == c1 || *c == c2) && !(*c == c1 && *c == c2),
        _ => false,
    }
}
