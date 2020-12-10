use std::fs;

fn main() {
    let cont = fs::read_to_string("day10.in").expect("Couldn't read file");
    let mut adapters = cont
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let ad_diff: Vec<u64> = adapters
        .iter()
        .scan(0u64, |prev, &ad| {
            let ret = ad - *prev;
            *prev = ad;
            Some(ret)
        })
        .collect();
    let num_1 = ad_diff.iter().filter(|&&x| x == 1).count();
    let num_3 = ad_diff.iter().filter(|&&x| x == 3).count();
    let mut comb =vec![0u64;*adapters.last().unwrap() as usize +1];
    let mut iter =adapters.iter().rev();
    
    comb[*iter.next().unwrap() as usize]=1;

    for &adapter in adapters.iter().rev().skip(1) {
        comb[adapter as usize] = comb[(adapter as usize+1)..=(adapter as usize +3)].iter().sum();
    }
    comb[0] = comb[1..=3].iter().sum();
    println!("prod: {}", num_1 * num_3);
    println!("comb: {}",comb[0])
}
