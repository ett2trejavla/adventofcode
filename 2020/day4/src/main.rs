use std::collections::HashMap;
use std::fs;
fn main() {
    let cont = fs::read_to_string("day4.in").expect("Couldn't read file");
    let possible_passports: Vec<&str> = cont
        .split(&"\n\n")
        .collect::<Vec<&str>>();
    let num_valid = possible_passports.iter().map(parse_pass).filter(valid_password).count();
    println!("valid: {}",num_valid)
}

fn parse_pass(pos_pass:&&str) ->HashMap<String,String>{
    let mut pass = HashMap::new();
    for field in pos_pass.split_whitespace().collect::<Vec<&str>>() {
        pass.insert(field[0..3].to_string(), field[4..].to_string());
    }
    
    pass
} 

fn valid_password(pos_pass: &HashMap<String,String>)-> bool{
    let fields = ["ecl","pid", "eyr","hcl","byr","iyr",  "cid","hgt"];
    for &field in &fields {
        match pos_pass.get(field) {
            Some(_) => {},
            None => if field != "cid" {return false;}, 
        }
    }
    true
}