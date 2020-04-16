use std::env;
use std::fs;
use std::io;

static PART2: bool = true;

fn main() {
    let input: &String = &env::args().nth(1).unwrap();
    
    let input_data = read_input(&input).unwrap();
    let result = calc_fuel(&input_data);

    println!("Total fuel: {}", result);
}

fn read_input(path: &str) -> io::Result<Vec<i32>> {
    let contents = fs::read_to_string(path)?;
    let numbers: Vec<i32> = contents.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    Ok(numbers)
}

fn calc_module_fuel(input: i32) -> i32 {
    (input / 3) - 2
}

fn calc_fuel(input_data: &Vec<i32>) -> i32 {
    let mut total_fuel: i32 = 0;

    for n in input_data.iter() {
        let fuel = calc_module_fuel(*n);
        total_fuel = total_fuel + fuel;
        if PART2 {
            total_fuel = total_fuel + calc_additional_fuel(fuel);
        }
    }

    total_fuel
}

fn calc_additional_fuel(input: i32) -> i32 {
    let mut total_fuel = 0;
    let mut additional_fuel = calc_module_fuel(input);
    while additional_fuel > 0 {
        total_fuel = total_fuel + additional_fuel;
        additional_fuel = calc_module_fuel(additional_fuel);
    }

    total_fuel
}