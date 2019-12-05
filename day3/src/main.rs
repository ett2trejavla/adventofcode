use std::cmp;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input.in").expect("Something went wrong reading the file");
    let sak: Vec<&str> = contents.split('\n').collect();
    let dist = closest_crossing(sak[0], sak[1]);
    println!("dist: {}", dist);
    let dist_2 = closest_crossing_part2(sak[0], sak[1]);
    println!("dist2: {}", dist_2)
}

enum Move {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}
fn paths(path: &str) -> Vec<(i32, i32)> {
    let mut p: Vec<(i32, i32)> = path
        .split(',')
        .map(parse_move)
        .scan((0, 0), cable_corners)
        .collect();
    p.insert(0, (0, 0));
    p
}

fn crossings(f_corners: &[(i32, i32)], s_corners: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut cross_points: Vec<(i32, i32)> = Vec::new();

    for ((fx1, fy1), (fx2, fy2)) in f_corners.iter().zip(f_corners.iter().skip(1)) {
        for ((sx1, sy1), (sx2, sy2)) in s_corners.iter().zip(s_corners.iter().skip(1)) {
            if fx1 == fx2 && sy1 == sy2 {
                let x_min = cmp::min(sx1, sx2);
                let x_max = cmp::max(sx1, sx2);
                let y_min = cmp::min(fy1, fy2);
                let y_max = cmp::max(fy1, fy2);
                if x_min <= fx1 && fx1 <= x_max && y_min <= sy1 && sy1 <= y_max {
                    cross_points.push((*fx1, *sy1))
                }
            } else if sx1 == sx2 && fy1 == fy2 {
                let x_min = cmp::min(fx1, fx2);
                let x_max = cmp::max(fx1, fx2);
                let y_min = cmp::min(sy1, sy2);
                let y_max = cmp::max(sy1, sy2);
                if x_min <= sx1 && sx1 <= x_max && y_min <= fy1 && fy1 <= y_max {
                    cross_points.push((*sx1, *fy1))
                }
            }
        }
    }
    //println!("{:?}", cross_points);
    cross_points
}
fn closest_crossing(first: &str, second: &str) -> i32 {
    let cable_1 = paths(first);
    let cable_2 = paths(second);

    let cross_points = crossings(&cable_1, &cable_2);
    cross_points
        .iter()
        .fold(std::i32::MAX, |acc: i32, coord: &(i32, i32)| match coord {
            (x, y) if *x == 0 && *y == 0 => acc,
            (x, y) => cmp::min(x.abs() + y.abs(), acc),
        })
}
fn closest_crossing_part2(first: &str, second: &str) -> i32 {
    let cable_1 = paths(first);
    let cable_2 = paths(second);

    let cross_points = crossings(&cable_1, &cable_2);
    cross_points
        .iter()
        .map(|coord: &(i32, i32)| dist_along(&cable_1, *coord) + dist_along(&cable_2, *coord))
        .fold(std::i32::MAX, |acc: i32, dist: i32| {
            if dist > 0 {
                cmp::min(acc, dist)
            } else {
                acc
            }
        })
}
fn dist_along(path: &[(i32, i32)], point: (i32, i32)) -> i32 {
    let mut dist: i32 = 0;
    let mut cur_pos: (i32, i32) = (0, 0);
    for (corner_x, corner_y) in path.iter().skip(1) {
        match point {
            (x, y) if x == cur_pos.0 => {
                let y_min = cmp::min(cur_pos.1, *corner_y);
                let y_max = cmp::max(cur_pos.1, *corner_y);
                if y_min <= y && y <= y_max {
                    return dist + (y - cur_pos.1).abs();
                }
            }
            (x, y) if y == cur_pos.1 => {
                let x_min = cmp::min(cur_pos.0, *corner_x);
                let x_max = cmp::max(cur_pos.0, *corner_x);
                if x_min <= x && x <= x_max {
                    return dist + (x - cur_pos.0).abs();
                }
            }
            (_, _) => {
                if *corner_x == cur_pos.0 {
                    dist += (*corner_y - cur_pos.1).abs();
                    cur_pos.1 = *corner_y;
                } else if *corner_y == cur_pos.1 {
                    dist += (*corner_x - cur_pos.0).abs();
                    cur_pos.0 = *corner_x;
                } else {
                    panic!("should not happen");
                }
            }
        }
    }
    panic!("could not find point");
}

fn parse_move(mov: &str) -> Move {
    let step_size: i32 = mov[1..].parse().unwrap();
    match mov.chars().nth(0) {
        Some('R') => Move::Right(step_size),
        Some('L') => Move::Left(step_size),
        Some('U') => Move::Up(step_size),
        Some('D') => Move::Down(step_size),
        Some(c) => panic!("Bad input, not a move, char {}", c),
        None => panic!("Bad input, not a move"),
    }
}

fn cable_corners(pos: &mut (i32, i32), next_move: Move) -> Option<(i32, i32)> {
    match next_move {
        Move::Right(step) => pos.0 += step,
        Move::Left(step) => pos.0 -= step,
        Move::Up(step) => pos.1 += step,
        Move::Down(step) => pos.1 -= step,
    }
    Some(*pos)
}

#[test]
fn simple_test() {
    let first_cable: String = String::from("R8,U5,L5,D3");
    let second_cable: String = String::from("U7,R6,D4,L4");
    assert_eq!(6, closest_crossing(&first_cable, &second_cable))
}
#[test]
fn simple_test2() {
    let first_cable: String = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let second_cable: String = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(159, closest_crossing(&first_cable, &second_cable));
    assert_eq!(610, closest_crossing_part2(&first_cable, &second_cable))
}

#[test]
fn simple_test3() {
    let first_cable: String = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let second_cable: String = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(135, closest_crossing(&first_cable, &second_cable));
    assert_eq!(410, closest_crossing_part2(&first_cable, &second_cable))
}
