use std::collections::HashMap;
use std::fs;
fn main() {
    let input = fs::read_to_string("input.in").expect("Something went wrong reading the file");
    let ore = required_ore(&input,1);
    println!("{}", ore);
    let num_ore:u64=1_000_000_000_000;
    let fuel_h=5_000_000;
    let fuel_l=500_000;
    let req_ore_l = required_ore(&input,fuel_l);
    let req_ore_h = required_ore(&input,fuel_h);
    let derv =(fuel_h-fuel_l) as f64/(req_ore_h-req_ore_l)as f64;
    println!("{}:{},{},{},{}",derv,req_ore_h,req_ore_l,fuel_h,fuel_l);

    let est_fuel=fuel_l+((num_ore-req_ore_l)as f64*derv )as u64 ;
    println!("{}:{}",est_fuel,required_ore(&input,est_fuel))
}

fn required_ore(input: &str,factor:u64) -> u64 {
    let reactions = parse_reactions(input);
    let mut elements = Vec::new();
    let mut excess_elements: HashMap<&String, u64> = HashMap::new();
    let (_, initial_reac) = reactions.get("FUEL").unwrap();
    for (num, elem) in initial_reac.iter() {
        elements.push((elem, num * factor));
    }
    let mut num_ore = 0;
    elements.sort_by(|(ae, an), (be, bn)| {
        let (areq_elem, _) = reactions.get(*ae).unwrap();
        let (breq_elem, _) = reactions.get(*be).unwrap();
        (an % areq_elem).partial_cmp(&(bn % breq_elem)).unwrap()
    });
    while let Some((elem, mut amount)) = elements.first() {
        match excess_elements.get_mut(elem) {
            Some(i) => {
                if amount >= *i {
                    amount -= *i;
                    *i = 0;
                } else {
                    *i -= amount;
                    elements.remove(0);
                    continue;
                }
            }
            None => (),
        };
        let (req_elem, new_elems) = reactions.get(*elem).unwrap();
        let num_reac = amount / req_elem
            + if amount % req_elem != 0 {
                *excess_elements.entry(elem).or_insert(0) += req_elem - (amount % req_elem);
                1
            } else {
                0
            };

        for (num_new_elem_p_reac, new_elem) in new_elems.iter() {
            if new_elem == "ORE" {
                num_ore += num_new_elem_p_reac * num_reac;
                continue;
            }
            elements.push((new_elem, num_reac * num_new_elem_p_reac));
        }
        elements.remove(0);
        elements.sort_by(|(ae, an), (be, bn)| {
            let (areq_elem, _) = reactions.get(*ae).unwrap();
            let (breq_elem, _) = reactions.get(*be).unwrap();
            (an % areq_elem).partial_cmp(&(bn % breq_elem)).unwrap()
        });
    }

    num_ore
}

fn parse_reactions(input: &str) -> HashMap<String, (u64, Vec<(u64, String)>)> {
    let mut reac = HashMap::new();

    let reactions: Vec<&str> = input.split('\n').collect();
    for reaction in reactions.iter() {
        let r: Vec<&str> = reaction.split("=>").collect();
        let (numout, elemout) = parse_element_amount(r[1]);
        let rin = r[0]
            .split(',')
            .map(parse_element_amount)
            .collect::<Vec<(u64, String)>>();
        reac.insert(elemout, (numout, rin));
    }
    reac
}
fn parse_element_amount(input: &str) -> (u64, String) {
    let x: Vec<&str> = input.trim().split(' ').collect();
    (x[0].parse().unwrap(), x[1].to_string())
}

#[test]
fn short_test() {
    let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
    assert_eq!(required_ore(&input,1), 31)
}
#[test]
fn medium_test1() {
    let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    assert_eq!(required_ore(&input,1), 13312)
}

#[test]
fn medium_test2() {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";
    assert_eq!(required_ore(&input,1), 180697)
}
#[test]
fn long_test() {
    let input = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";
    assert_eq!(required_ore(&input,1), 2210736)
}
