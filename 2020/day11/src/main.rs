use std::fs;

fn main() {
    let cont = fs::read_to_string("day11.in").expect("Couldn't read file");
    let mut seats: Vec<_> = cont
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    loop {
        let (num_changed, new_seats) = update_seats(&seats);
        seats=new_seats;
        if num_changed == 0 {
            let occupied_seats: usize = seats
                .iter()
                .map(|line| line.iter().filter(|&&c| c == '#').count())
                .sum();
            println!("occupied seats: {}", occupied_seats);
            break;
        }
    }
}

fn update_seats(seats: &[Vec<char>]) -> (u32, Vec<Vec<char>>) {
    let mut new_seats = seats.clone().to_vec();
    let mut updated_seats = 0;
    for (i, line) in new_seats.iter_mut().enumerate() {
        for (j, c) in line.iter_mut().enumerate() {
            match *c {
                'L' => {
                    if count_neigbors_2(&seats, i as isize, j as isize) == 0 {
                        *c = '#';
                        updated_seats += 1;
                    }
                }
                '#' => {
                    if count_neigbors_2(&seats, i as isize, j as isize) >= 5 {
                        *c = 'L';
                        updated_seats += 1;
                    }
                }
                _ => {}
            }
        }
    }
    (updated_seats, new_seats)
}

fn count_neigbors_1(seats: &[Vec<char>], i: isize, j: isize) -> usize {
    let mut delta: Vec<(isize, isize)> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if (i, j) != (0, 0) {
                delta.push((i as isize, j as isize));
            }
        }
    }
    delta
        .iter()
        .filter(|(di, dj)| {
            match seats
                .iter()
                .nth((i + di) as usize)
                .and_then(|line|line.iter()
                .nth((j + dj) as usize))
            {
                Some('L') => false,
                Some('#') => true,
                _ => false,
            }
        })
        .count()
}


fn count_neigbors_2(seats: &[Vec<char>], i: isize, j: isize) -> usize {
    let mut dir: Vec<(isize, isize)> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if (i, j) != (0, 0) {
                dir.push((i as isize, j as isize));
            }
        }
    }
    dir
        .iter()
        .filter(|(x_hat, y_hat)| {
            let mut n=1;
            loop{
            match seats
                .iter()
                .nth((i + (n* *x_hat) ) as usize)
                .and_then(|line|line.iter()
                .nth((j + (n**y_hat)) as usize))
            {
                Some('L') => {return false;},
                Some('#') =>  {return true;},
                Some('.') => {n+=1;}
                _ => {return false;},
            }
        }
        })
        .count()
}

