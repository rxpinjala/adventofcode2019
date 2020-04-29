use std::env;
use std::fs;
use std::io;
use intcode;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let input_data = read_input(&input_path).unwrap();

    let mut p1 = input_data.clone();
    p1[1] = 12;
    p1[2] = 2;
    intcode::run(&mut p1, 0).unwrap();
    println!("Part 1 result: {}", p1[0]);

    for i in 0..100 {
        for j in 0..100 {
            let mut p2 = input_data.clone();
            p2[1] = i;
            p2[2] = j;
            if let Ok(()) = intcode::run(&mut p2, 0) {
                if p2[0] == 19690720 {
                    println!("Part 2 result: {}", i * 100 + j);
                    break;
                }
            }

        }
    }
}

fn read_input(path: &str) -> io::Result<Vec<i32>> {
    let contents = fs::read_to_string(path)?;
    let numbers: Vec<i32> = contents.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    Ok(numbers)
}

