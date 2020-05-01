use std::env;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let p = intcode::read_from_path(input_path).unwrap();

    let mut p1 = p.clone();
    let output = intcode::run(&mut p1, 0, vec!(1)).unwrap();
    dbg!(output);

    let mut p2 = p.clone();
    let output = intcode::run(&mut p2, 0, vec!(5)).unwrap();
    dbg!(output);
}