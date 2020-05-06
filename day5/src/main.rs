use std::env;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let p = intcode::read_from_path(input_path).unwrap();

    let mut c1 = intcode::Computer::new(p.clone());
    c1.input.push_back(1);
    let output = c1.run().unwrap();
    dbg!(output);

    let mut c2 = intcode::Computer::new(p.clone());
    c2.input.push_back(5);
    let output = c2.run().unwrap();
    dbg!(output);
}