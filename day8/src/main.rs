use std::fs;

fn main() {
    let filename = "input.in";

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let output = parse_input(&input, (6, 25));
    println!("reached end: {}", output);
    let image = decode_image(&input, (6, 25)).chunks(25).map(|s: &[char]| s.iter().collect::<String>())
    .collect::<Vec<String>>()
    .join("\n");
    
    println!("{}", image);
}

fn parse_input(input: &str, (rows, cols): (usize, usize)) -> u32 {
    let layers = input.chars().collect::<Vec<_>>();
    let layer: &[char] = layers
        .chunks(rows * cols)
        .min_by(|x, y| num_char(x, '0').cmp(&num_char(y, '0')))
        .unwrap();
    num_char(layer, '1') * num_char(layer, '2')
}
fn num_char(layer: &[char], c: char) -> u32 {
    layer.iter().filter(|x| **x == c).count() as u32
}

fn decode_image(input: &str, (rows, cols): (usize, usize)) -> Vec<char> {
    let layers = input.chars().collect::<Vec<_>>();
    let mut image: Vec<char> = Vec::new();
    for pixels in 0..rows * cols {
        let pixel = layers
            .iter()
            .skip(pixels)
            .step_by(rows * cols)
            .find(|&&x| x == '0' || x == '1')
            .unwrap();
        image.push(match *pixel{
            '0' =>'.',
            '1' => '8',
            _ => panic!("not possible")
        });
    }
    image
}

#[test]
fn small_input() {
    let input = "123456789012";
    assert_eq!(1, parse_input(input, (2, 3)))
}
#[test]
fn small_p2() {
    let input = "0222112222120000";
    let image = decode_image(&input, (2, 2));
    assert_eq!(image, vec!['0', '1', '1', '0'])
}
