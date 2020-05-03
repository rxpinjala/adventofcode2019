use std::env;
use itertools::Itertools;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let p = intcode::read_from_path(input_path).unwrap();

    let phases: Vec<i32> = vec!(0, 1, 2, 3, 4);
    let mut best: i32 = 0;
    for permutation in phases.into_iter().permutations(5) {
        let result = run_program_on_amps(&p, &permutation[..]).unwrap();

        if result > best {
            best = result;
        }
    }
    println!("Part 1: {}", best);
}

fn run_program_on_amps(p: &[i32], phases: &[i32]) -> Result<i32, &'static str> {
    let mut input: i32 = 0;

    for phase in phases.iter() {
        let mut p = p.to_vec();
        let output = intcode::run(&mut p, 0, vec!(*phase, input))?;
        assert!(output.len() == 1);
        input = output[0];
    }

    Ok(input)
}

#[test]
fn provided_test_1() {
    let p = intcode::read_from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let result = run_program_on_amps(&p, &vec!(4,3,2,1,0));
    assert_eq!(result.unwrap(), 43210);
}
