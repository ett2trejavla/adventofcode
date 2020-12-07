use std::collections::HashMap;
use std::fs;

struct BagRules(HashMap<String, u32>);

fn main() {
    let cont = fs::read_to_string("day7.in").expect("Couldn't read file");
    let bags: HashMap<String, BagRules> = cont
        .split('\n')
        .map(|bag_string| {
            let mut bag_iter = bag_string.split(&"contain");
            let bag_name = bag_iter.next().unwrap().trim().trim_end_matches('s');
            let bag_rules_str = bag_iter.next().unwrap();
            let bag_rules = if bag_rules_str.trim() == "no other bags." {
                BagRules(HashMap::new())
            } else {
                BagRules(
                    bag_rules_str[..(bag_rules_str.len() - 1)]
                        .split(',')
                        .map(|rule| {
                            let num: u32 = rule.chars().nth(1).unwrap().to_digit(10).unwrap();
                            let bag = &rule[3..].trim_end_matches('s');
                            (bag.to_string(), num)
                        })
                        .collect::<HashMap<String, u32>>(),
                )
            };
            (bag_name.to_string(), bag_rules)
        })
        .collect();
    let mut contains_shiny_gold_memo =HashMap::new();
    contains_shiny_gold_memo.insert("shiny gold bag".to_string(), true);

    let count =bags.keys().filter(|&bag| contains_shiny_gold(&mut contains_shiny_gold_memo, bag.to_string(), &bags)).count();
    println!("bags that can contain a shiny gold bag: {}",count-1);
    let bags_in_shiny = count_bags("shiny gold bag".to_string(), &bags);
    println!("A shiny gold bag countains: {}",bags_in_shiny-1);
}

fn contains_shiny_gold(mut memo:&mut HashMap<String,bool>,bag:String, bag_rules:&HashMap<String,BagRules>)->bool{
    match memo.get(&bag){
        Some(b) => *b,
        None =>{
        let rules =bag_rules.get(&bag).unwrap();
        let mut can_contain:bool =false;
        for  contained_bag in rules.0.keys(){
            can_contain |= contains_shiny_gold(&mut memo, contained_bag.to_string(), &bag_rules);
        }
        memo.insert(bag, can_contain);
        can_contain
        }
    }
}

fn count_bags(bag:String, bag_rules:&HashMap<String,BagRules>)->u32{
    let rules =bag_rules.get(&bag).unwrap();
    let mut count =1u32;
    for (containd_bag,&num_bags) in rules.0.iter() {
        count += num_bags * count_bags(containd_bag.clone(),&bag_rules);
    }
    count
}