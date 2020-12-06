#![feature(iterator_fold_self)]
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let cont = fs::read_to_string("day6.in").expect("Couldn't read file");
    let group_custums_dec: Vec<&str> = cont.split(&"\n\n").collect::<Vec<&str>>();
    sol_1(&group_custums_dec);
    sol_2(group_custums_dec)
}

fn sol_1(group_custums_dec: &[&str]) {
    let sum_customs_dec: usize = group_custums_dec
        .iter()
        .map(|s| s.chars().filter(|&c| c != '\n').unique().count())
        .sum();

    println!("number of customs declarations {}", sum_customs_dec)
}

fn sol_2(group_custums_dec: Vec<&str>) {
    let sum_customs_dec: usize = group_custums_dec
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .into_iter()
                .fold_first(|acc, x| acc.intersection(&x).cloned().collect())
        })
        .map(|set| set.unwrap().len())
        .sum();

    println!("number of customs declarations {}", sum_customs_dec)
}
