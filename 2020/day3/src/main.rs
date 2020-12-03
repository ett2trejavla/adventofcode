
use std::fs;

fn main() {
    let cont = fs::read_to_string("day3.in").expect("Couldn't read file");
    let world_map: Vec<&str> = cont
        .split('\n')
        .collect::<Vec<&str>>();



    let slopes:Vec<(usize,usize)> =vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
    let mut tree_counts:Vec<i32> =vec![];

    for (delta_x,delta_y) in slopes {
        let mut xcoord =0usize;
        let mut ycoord =0usize;
        let mut trees = 0;
        for (world_y,world_row) in world_map.iter().enumerate() {
            if world_y == ycoord{
                match world_row.chars().nth(xcoord % world_row.len()) {
                    Some('#') =>trees+=1,
                    _    => {},
                }
                xcoord +=delta_x;
                ycoord +=delta_y;
            }
        }
        tree_counts.push(trees);
    }

    println!("trees: {}",tree_counts.iter().fold(1, |acc,x| acc*x))

}
