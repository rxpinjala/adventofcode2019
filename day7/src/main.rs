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

    let phases: Vec<i32> = vec!(5, 6, 7, 8, 9);
    let mut best: i32 = 0;
    for permutation in phases.into_iter().permutations(5) {
        let result = run_on_amps_with_feedback(&p, &permutation[..]).unwrap();

        if result > best {
            best = result;
        }
    }
    println!("Part 2: {}", best);
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

fn run_on_amps_with_feedback(p: &[i32], phases: &[i32]) -> Result<i32, &'static str> {
    type Amp = (Vec<i32>, intcode::PC);    
    let mut amps: Vec<Amp> = Vec::new();
    for _ in phases {
        amps.push((p.to_vec(), 0));
    }

    let mut next_phase = 0;
    for (i,amp) in amps.iter_mut().enumerate() {
        println!("Amp {}:", i);
        let run_result = intcode::run_step(&mut amp.0, amp.1, Some(phases[next_phase]))?;
        match run_result {
            intcode::RunResult::Done => return Err("Program exited unexpectedly"),
            intcode::RunResult::Output(_,_) => return Err("Program produced unexpected output"),
            intcode::RunResult::WaitingForInput(pc) => amp.1 = pc
        }
        next_phase = next_phase + 1;
    }

    let mut current_input: i32 = 0;
    let mut loop_limit = 1000;
    loop {
        for (i,amp) in amps.iter_mut().enumerate() {
            println!("Amp {}:", i);
            let run_result = intcode::run_step(&mut amp.0, amp.1, Some(current_input))?;
            match run_result {
                intcode::RunResult::Done => {
                    return Ok(current_input);
                }
                intcode::RunResult::Output(pc, value) => {
                    current_input = value;
                    amp.1 = pc;
                }
                intcode::RunResult::WaitingForInput(_pc) =>
                    return Err("Expected output, but program wanted input")
            }
        }

        loop_limit = loop_limit - 1;
        if loop_limit == 0 {
            return Err("Iteration limit reached")
        }
    }
}

#[test]
fn provided_test_1() {
    let p = intcode::read_from_string("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let result = run_program_on_amps(&p, &vec!(4,3,2,1,0));
    assert_eq!(result.unwrap(), 43210);
}

#[test]
fn provided_test_2() {
    let p = intcode::read_from_string("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    let result = run_on_amps_with_feedback(&p, &vec!(9,8,7,6,5));
    assert_eq!(result.unwrap(), 139629729);
}