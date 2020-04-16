use std::env;
use std::fs;
use std::io;

fn main() {
    let input: &String = &env::args().nth(1).unwrap();
    
    let input_data = read_input(&input).unwrap();
    let result = calculate_fuel(input_data);

    println!("Total fuel: {}", result);
}

fn read_input(path: &str) -> io::Result<Vec<i32>> {
    let contents = fs::read_to_string(path)?;
    let numbers: Vec<i32> = contents.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    Ok(numbers)
}

fn calculate_fuel(input_data: Vec<i32>) -> i32 {
    let mut total_fuel: i32 = 0;

    for n in input_data.iter() {
        let fuel = (n / 3) - 2;
        total_fuel = total_fuel + fuel;
    }

    total_fuel
}
