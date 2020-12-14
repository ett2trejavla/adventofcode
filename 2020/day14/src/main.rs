use std::fs;
use std::collections::HashMap;

fn main() {
    let cont = fs::read_to_string("day14.in").expect("Couldn't read file");
    let mut memory:HashMap<u64,u64> = HashMap::new();
    let writes: Vec<_> = cont
        .split(";\n")
        .map(|block| block.split('\n').collect::<Vec<&str>>())
        .collect();
    for block in writes{
        let mask = &block.iter().next().unwrap()[7..];
        let or_mask_str = mask.chars().clone().map(|c| match c {
            'X'=>'1',
            _ => '0'
        }).collect::<String>();
        let or_mask = u64::from_str_radix(&or_mask_str,2).unwrap();
        let and_mask_str = mask.chars().clone().map(|c| match c {
            '1'=>'1',
            _ => '0'
        }).collect::<String>();
        let and_mask = u64::from_str_radix(&and_mask_str,2).unwrap();
        for &written_line in block.iter().skip(1) {
            let mut line_iter =written_line.split('=');
            let address = line_iter.next().unwrap().trim().parse().unwrap();
            let value = line_iter.next().unwrap().trim().parse().unwrap();
            memory.insert(address,apply_mask(and_mask,or_mask,value));
        }
        
    }
    let mem_sum:u64 =memory.values().sum();
    println!("mem_sum: {}",mem_sum)
}
fn apply_mask(and_m:u64,or_m:u64, val:u64)->u64{
    (val & and_m) | (val|or_m)
}
