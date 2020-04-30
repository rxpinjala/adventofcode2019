use std::env;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let mut p = intcode::read_from_path(input_path).unwrap();

    let output = intcode::run(&mut p, 0, vec!(1)).unwrap();
    dbg!(output);
}