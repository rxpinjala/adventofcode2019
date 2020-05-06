use std::env;
use std::fs;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let input_data = fs::read_to_string(input_path).unwrap();
    
    let dimensions = 6 * 25;
    let mut layers: Vec<Vec<char>> = Vec::new();
    let mut i = 0;
    while i + dimensions < input_data.len() {
        layers.push(input_data[i..i+dimensions].chars().collect());
        i = i + dimensions;
    }

    let count_zeros = get_digit_counter('0');
    let fewest_zeros = layers.iter().enumerate().min_by_key(|(_, layer)| layer.iter().fold(0, &count_zeros)).unwrap();

    let result = fewest_zeros.1.iter().fold(0, get_digit_counter('1')) * fewest_zeros.1.iter().fold(0, get_digit_counter('2'));
    println!("Part 1: {}", result);

    layers.reverse();
    let final_image = layers.iter().skip(1).fold(layers[0].clone(), |image, layer| merge_layer(image, layer));
    
    for x in 0..6 {
        for y in 0..25 {
            let c: char = final_image[x * 25 + y];
            print!("{}", match c {
                '0' => 'X',
                '1' => ' ',
                _   => '?'
            });
        }
        println!("");
    }
}

fn get_digit_counter(digit: char) -> Box<dyn Fn(usize, &char) -> usize> {
    Box::new(move |count, ch: &char| if *ch == digit { count + 1 } else { count })
}

// c2 overlays c1
fn merge_pixel(c1: char, c2: char) -> char {
    match c2 {
        '0' => '0',
        '1' => '1',
        '2' => c1,
        _ => '?'
    }
}

fn merge_layer(mut image: Vec<char>, layer: &[char]) -> Vec<char> {
    image.iter_mut().zip(layer.iter()).for_each(|(c1, c2)| *c1 = merge_pixel(*c1, *c2));

    image
}