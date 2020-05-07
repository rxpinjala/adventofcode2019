use std::env;
use std::fs;
use std::io;
use intcode;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let input_data = read_input(&input_path).unwrap();

    let mut c = intcode::Computer::new(input_data.clone());
    c.mem[1] = 12;
    c.mem[2] = 2;
    c.run().unwrap();
    println!("Part 1 result: {}", c.mem[0]);

    for i in 0..100 {
        for j in 0..100 {
            let mut c = intcode::Computer::new(input_data.clone());
            c.mem[1] = i;
            c.mem[2] = j;
            if let Ok(_) = c.run() {
                if c.mem[0] == 19690720 {
                    println!("Part 2 result: {}", i * 100 + j);
                    break;
                }
            }

        }
    }
}

fn read_input(path: &str) -> io::Result<Vec<i64>> {
    let contents = fs::read_to_string(path)?;
    let numbers: Vec<i64> = contents.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    Ok(numbers)
}

