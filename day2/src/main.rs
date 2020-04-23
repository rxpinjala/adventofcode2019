use std::env;
use std::fs;
use std::io;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let input_data = read_input(&input_path).unwrap();

    let mut p1 = input_data.clone();
    p1[1] = 12;
    p1[2] = 2;
    run_intcode(&mut p1, 0).unwrap();
    println!("Part 1 result: {}", p1[0]);

    for i in 0..100 {
        for j in 0..100 {
            let mut p2 = input_data.clone();
            p2[1] = i;
            p2[2] = j;
            if let Ok(()) = run_intcode(&mut p2, 0) {
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

fn run_intcode(p: &mut Vec<i32>, start: usize) -> Result<(), &'static str> {
    let mut pc: usize = start;

    loop {
        match p.get(pc) {
            None => Err("Bad PC"),
            Some(1) => {
                let inst = get_inst(&p, pc)?;
                let target = inst[3] as usize;
                p[target] = p[inst[1] as usize] + p[inst[2] as usize];
                pc = pc + 4;
                Ok(())
            },
            Some(2) => {
                let inst = get_inst(&p, pc)?;
                let target = inst[3] as usize;
                p[target] = p[inst[1] as usize] * p[inst[2] as usize];
                pc = pc + 4;
                Ok(())
            }
            Some(99) => break,
            _ => Err("Unrecognized opcode")
        }?;
    }

    Ok(())
}

fn get_inst(p: &Vec<i32>, i: usize) -> Result<&[i32], &'static str> {
    let lim = p.len() as i32;
    if let Some(inst) = p.get(i..i + 4) {
        if inst[1] < lim && inst[2] < lim && inst[3] < lim {
            Ok(inst)
        } else {
            Err("Bad instruction")
        }
    } else {
        Err("Bad instruction")
    }
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn provided_case_1() {
    let mut p = vec!(1,9,10,3,2,3,11,0,99,30,40,50);
    run_intcode(&mut p, 0).unwrap();

    assert_eq!(p, vec!(3500,9,10,70,2,3,11,0,99,30,40,50));
}

#[test]
fn provided_case_2() {
    let mut p = vec!(1,0,0,0,99);
    run_intcode(&mut p, 0).unwrap();

    assert_eq!(p, vec!(2,0,0,0,99));
}

}