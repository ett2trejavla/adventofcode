use std::fs;

fn main() {
    let cont = fs::read_to_string("day9.in").expect("Couldn't read file");
    let xmasencryption = cont
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let faulty_num = solve_1(xmasencryption.clone());
    solve_2(faulty_num, xmasencryption);
}

fn solve_1(input: Vec<u64>) -> u64 {
    for &num in &input[25..] {
        if input
            .windows(25)
            .map(|win| search_triangle_product(win, win, |&win_i, &win_j| win_i + win_j == num))
            .find(|x| x.is_some())
            .is_none()
        {
            println!("holds until: {}", num);
            return num;
        }
    }
    0u64
}
fn solve_2(f_num: u64, input: Vec<u64>) {
    let sum_so_far: Vec<_> = input
        .iter()
        .scan(0u64, |acc, &x| {
            let ret =*acc;
            *acc += x;
            Some(ret)
        })
        .zip(input.iter().enumerate())
        .collect();
    let ((sum_i, (i_ind, &i_num)), (sum_j, (j_ind, &j_num))) =
        search_triangle_product(&sum_so_far, &sum_so_far, |(sum_i, _), (sum_j, _)| {
            *sum_j - *sum_i == f_num
        })
        .unwrap();
    let cont_range = &input[(*i_ind as usize)..(*j_ind as usize)];
    let min =cont_range.iter().min().unwrap();
    let max =cont_range.iter().max().unwrap();
        println!(
        "range {}-{}, sum: {}",
        i_num,
        j_num,
        min+max,
    )
}

fn search_triangle_product<'a, 'b, T, P>(
    a: &'a [T],
    b: &'b [T],
    predicate: P,
) -> Option<(&'a T, &'b T)>
where
    P: Fn(&T, &T) -> bool,
{
    for (i, a_i) in a.iter().enumerate() {
        for b_i in b.iter().skip(i + 1) {
            if predicate(a_i, b_i) {
                return Some((a_i, b_i));
            }
        }
    }
    None
}
