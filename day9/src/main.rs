use std::env;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();

    let mut c = intcode::Computer::load_from_path(input_path).unwrap();
    c.input.push_back(1);
    c.run().unwrap();
    println!("Part 1: {}", c.take_output()[0]);

    let mut c = intcode::Computer::load_from_path(input_path).unwrap();
    c.input.push_back(2);
    c.run().unwrap();
    println!("Part 2: {}", c.take_output()[0]);
}
