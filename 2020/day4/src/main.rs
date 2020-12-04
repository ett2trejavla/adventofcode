use std::collections::HashMap;
use std::fs;
fn main() {
    let cont = fs::read_to_string("day4.in").expect("Couldn't read file");
    let possible_passports: Vec<&str> = cont.split(&"\n\n").collect::<Vec<&str>>();
    let num_valid = possible_passports
        .iter()
        .map(parse_pass)
        .filter(valid_password)
        .count();
    println!("valid: {}", num_valid)
}

fn parse_pass(pos_pass: &&str) -> HashMap<String, String> {
    let mut pass = HashMap::new();
    for field in pos_pass.split_whitespace().collect::<Vec<&str>>() {
        pass.insert(field[0..3].to_string(), field[4..].to_string());
    }

    pass
}

fn valid_password(pos_pass: &HashMap<String, String>) -> bool {
    let fields = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"];
    let validators = [
        validate_ecl,
        validate_pid,
        validate_eyr,
        validate_hcl,
        validate_byr,
        validate_iyr,
        validate_cid,
        validate_hgt,
    ];
    for (&field, validator) in fields.iter().zip(validators.iter()) {
        match pos_pass.get(field) {
            Some(value) => {
                if !validator(value) {
                    return false;
                }
            }
            None => {
                if field != "cid" {
                    return false;
                }
            }
        }
    }
    true
}

fn validate_cid(_: &str) -> bool {
    true
}
fn validate_byr(value: &str) -> bool {
    value
        .parse::<u32>()
        .map_or(None, |x| match x {
            1920..=2002 => Some(x),
            _ => None,
        })
        .is_some()
}

fn validate_eyr(value: &str) -> bool {
    value
        .parse::<u32>()
        .map_or(None, |x| match x {
            2020..=2030 => Some(x),
            _ => None,
        })
        .is_some()
}
fn validate_iyr(value: &str) -> bool {
    value
        .parse::<u32>()
        .map_or(None, |x| match x {
            2010..=2020 => Some(x),
            _ => None,
        })
        .is_some()
}
fn validate_pid(value: &str) -> bool {
    value.chars().count() == 9 && value.chars().filter(|&c| c.is_numeric()).count() == 9
}

fn validate_hgt(value: &str) -> bool {
    if value.ends_with("in") {
        value[..value.len() - 2]
            .parse::<u32>()
            .map_or(None, |x| match x {
                59..=76 => Some(x),
                _ => None,
            })
            .is_some()
    } else if value.ends_with("cm") {
        value[..value.len() - 2]
            .parse::<u32>()
            .map_or(None, |x| match x {
                150..=193 => Some(x),
                _ => None,
            })
            .is_some()
    } else {
        false
    }
}

fn validate_hcl(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("#")
        && value
            .chars()
            .step_by(1)
            .filter(|&c| c.is_ascii_hexdigit())
            .count()
            == 6
}

fn validate_ecl(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

#[test]
fn test_validate_byr() {
    assert!(validate_byr("1980"));
    assert!(validate_byr("1920"));
    assert!(validate_byr("2002"));
    assert!(!validate_byr("1900"))
}
