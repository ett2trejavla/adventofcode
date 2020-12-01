use num_traits::Num;
use std::fs;

fn main() {
    let cont = fs::read_to_string("day1.in").expect("Couldn't read file");
    let expenses = cont
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    //solve_1(expenses)
    solve_2(expenses)
}

fn solve_1(expenses: Vec<u32>) {
    let (a, b) = search_triangle_product(&expenses, &expenses, |x, y| x + y == 2020).unwrap();
    println!("a : {}, b: {}, a*b: {}", a, b, a * b)
}

fn solve_2(expenses: Vec<u32>) {
    for (i, expense) in expenses.iter().enumerate() {
        match search_triangle_product(&expenses[i..], &expenses[i..], |x, y| {
            expense + x + y == 2020
        }) {
            None => continue,
            Some((b, c)) => {
                println!(
                    "a : {}, b: {},c: {}, a*b*c: {}",
                    expense,
                    b,
                    c,
                    expense * b * c
                )
            }
        }
    }
}

fn search_triangle_product<'a, 'b, T: Num, P: Fn(&T, &T) -> bool>(
    a: &'a [T],
    b: &'b [T],
    predicate: P,
) -> Option<(&'a T, &'b T)> {
    for (i, a_i) in a.iter().enumerate() {
        for b_i in b.iter().skip(i + 1) {
            if predicate(a_i, b_i) {
                return Some((a_i, b_i));
            }
        }
    }
    None
}
