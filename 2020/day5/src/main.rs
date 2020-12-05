use std::fs;
fn main() {
    let cont = fs::read_to_string("day5.in").expect("Couldn't read file");
    let specifications: Vec<&str> = cont.split('\n').collect::<Vec<&str>>();
    let specifications_bin = specifications.iter().map(to_binary).collect::<Vec<_>>();
    let mut ids: Vec<u16> = specifications_bin
        .iter()
        .map(|spec| (&spec[0..7], &spec[7..]))
        .map(|(x, y)| {
            (
                u16::from_str_radix(x, 2).unwrap(),
                u16::from_str_radix(y, 2).unwrap(),
            )
        })
        .map(|(x, y)| 8 * x + y)
        .collect();

    sol_1(&ids);
    ids.sort();
    sol_2(&ids);
}

fn to_binary(spec: &&str) -> String {
    spec.chars()
        .map(|letter| match letter {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            l => panic!("Illegal letter in spec: {}", l),
        })
        .collect()
}

fn sol_1(ids: &[u16]) {
    let max_id = ids.iter().max().unwrap();

    println!("Max Id: {}", max_id)
}
fn sol_2(ids: &[u16]) {
    let my_id = ids
        .iter()
        .zip(ids.iter().skip(1))
        .find(|(&id_l, &id_r)| id_r - id_l == 2)
        .map(|neib_ids| *neib_ids.0 + 1)
        .unwrap();

    println!("My Id: {}", my_id)
}
