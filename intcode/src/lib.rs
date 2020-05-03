
#[derive(Debug, Eq, PartialEq)]
pub enum InParam {
    Position(i32),
    Immediate(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub enum OutParam {
    Position(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Inst {
    Add(InParam, InParam, OutParam),
    Mult(InParam, InParam, OutParam),
    Input(OutParam),
    Output(InParam),
    JumpIfTrue(InParam, InParam),
    JumpIfFalse(InParam, InParam),
    LessThan(InParam, InParam, OutParam),
    Equal(InParam, InParam, OutParam),
    Exit,
}

impl Inst {
    fn len(&self) -> usize {
        match *self {
            Inst::Add(_,_,_) => 4,
            Inst::Mult(_,_,_) => 4,
            Inst::Exit => 1,
            Inst::Input(_) => 2,
            Inst::Output(_) => 2,
            Inst::JumpIfTrue(_,_) => 3,
            Inst::JumpIfFalse(_,_) => 3,
            Inst::LessThan(_,_,_) => 4,
            Inst::Equal(_,_,_) => 4,
        }
    }
}

pub fn decode(p: &[i32], pc: usize) -> Result<Inst, &'static str> {
    if pc >= p.len() {
        return Err("Bad PC");
    }

    let opcode = p[pc] % 100;
    let modes = p[pc] / 100;
    match opcode {
        1 => { // add
            let inst = p.get(pc..pc + 4).ok_or("Bad add instruction")?;
            match modes {
                0 => Ok(Inst::Add(InParam::Position(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                1 => Ok(Inst::Add(InParam::Immediate(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                10 => Ok(Inst::Add(InParam::Position(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                11 => Ok(Inst::Add(InParam::Immediate(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                _ => Err("Bad add instruction modes")
            }
        }

        2 => { // multiply
            let inst = p.get(pc..pc + 4).ok_or("Bad mul instruction")?;
            match modes {
                0 => Ok(Inst::Mult(InParam::Position(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                1 => Ok(Inst::Mult(InParam::Immediate(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                10 => Ok(Inst::Mult(InParam::Position(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                11 => Ok(Inst::Mult(InParam::Immediate(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                _ => Err("Bad mult instruction modes")
            }
        }

        3 => { // input
            let inst = p.get(pc..pc + 2).ok_or("Bad input instruction")?;
            match modes {
                0 => Ok(Inst::Input(OutParam::Position(inst[1]))),
                _ => Err("Bad mult instruction modes")
            }
        }

        4 => { // output
            let inst = p.get(pc..pc + 2).ok_or("Bad output instruction")?;
            match modes {
                0 => Ok(Inst::Output(InParam::Position(inst[1]))),
                1 => Ok(Inst::Output(InParam::Immediate(inst[1]))),
                _ => Err("Bad mult instruction modes")
            }
        }

        5 => { // jump if true
            let inst = p.get(pc..pc + 3).ok_or("Bad jump-if-true instruction")?;
            match modes {
                0 => Ok(Inst::JumpIfTrue(InParam::Position(inst[1]), InParam::Position(inst[2]))),
                1 => Ok(Inst::JumpIfTrue(InParam::Immediate(inst[1]), InParam::Position(inst[2]))),
                10 => Ok(Inst::JumpIfTrue(InParam::Position(inst[1]), InParam::Immediate(inst[2]))),
                11 => Ok(Inst::JumpIfTrue(InParam::Immediate(inst[1]), InParam::Immediate(inst[2]))),
                _ => Err("Bad jump-if-true instruction modes")
            }
        }

        6 => { // jump if false
            let inst = p.get(pc..pc + 3).ok_or("Bad jump-if-false instruction")?;
            match modes {
                0 => Ok(Inst::JumpIfFalse(InParam::Position(inst[1]), InParam::Position(inst[2]))),
                1 => Ok(Inst::JumpIfFalse(InParam::Immediate(inst[1]), InParam::Position(inst[2]))),
                10 => Ok(Inst::JumpIfFalse(InParam::Position(inst[1]), InParam::Immediate(inst[2]))),
                11 => Ok(Inst::JumpIfFalse(InParam::Immediate(inst[1]), InParam::Immediate(inst[2]))),
                _ => Err("Bad jump-if-false instruction modes")
            }
        }

        7 => { // less than
            let inst = p.get(pc..pc + 4).ok_or("Bad less-than instruction")?;
            match modes {
                0 => Ok(Inst::LessThan(InParam::Position(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                1 => Ok(Inst::LessThan(InParam::Immediate(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                10 => Ok(Inst::LessThan(InParam::Position(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                11 => Ok(Inst::LessThan(InParam::Immediate(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                _ => Err("Bad less-than instruction modes")
            }
        }

        8 => { // equal
            let inst = p.get(pc..pc + 4).ok_or("Bad equal instruction")?;
            match modes {
                0 => Ok(Inst::Equal(InParam::Position(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                1 => Ok(Inst::Equal(InParam::Immediate(inst[1]), InParam::Position(inst[2]), OutParam::Position(inst[3]))),
                10 => Ok(Inst::Equal(InParam::Position(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                11 => Ok(Inst::Equal(InParam::Immediate(inst[1]), InParam::Immediate(inst[2]), OutParam::Position(inst[3]))),
                _ => Err("Bad equal instruction modes")
            }
        }

        99 => Ok(Inst::Exit),

        _ => Err("Bad instruction")
    }
}

#[test]
fn test_decode_add() {
    let inst = decode(&vec!(1,2,3,4)[..], 0).unwrap();
    assert_eq!(inst, Inst::Add(InParam::Position(2), InParam::Position(3), OutParam::Position(4)));

    let inst = decode(&vec!(101,2,3,4)[..], 0).unwrap();
    assert_eq!(inst, Inst::Add(InParam::Immediate(2), InParam::Position(3), OutParam::Position(4)));

    let inst = decode(&vec!(1001,2,3,4)[..], 0).unwrap();
    assert_eq!(inst, Inst::Add(InParam::Position(2), InParam::Immediate(3), OutParam::Position(4)));
}

#[test]
fn test_decode_mult() {
    let inst = decode(&vec!(2,2,3,4)[..], 0).unwrap();
    assert_eq!(inst, Inst::Mult(InParam::Position(2), InParam::Position(3), OutParam::Position(4)));

    let inst = decode(&vec!(102,2,3,4)[..], 0).unwrap();
    assert_eq!(inst, Inst::Mult(InParam::Immediate(2), InParam::Position(3), OutParam::Position(4)));

    let inst = decode(&vec!(1002,2,3,4)[..], 0).unwrap();
    assert_eq!(inst, Inst::Mult(InParam::Position(2), InParam::Immediate(3), OutParam::Position(4)));
}

#[test]
fn test_decode_input() {
    let inst = decode(&vec!(3,0)[..], 0).unwrap();
    assert_eq!(inst, Inst::Input(OutParam::Position(0)));
}

#[test]
fn test_decode_output() {
    let inst = decode(&vec!(4,0)[..], 0).unwrap();
    assert_eq!(inst, Inst::Output(InParam::Position(0)));

    let inst = decode(&vec!(104,0)[..], 0).unwrap();
    assert_eq!(inst, Inst::Output(InParam::Immediate(0)));
}

#[test]
fn test_decode_jumpiftrue() {
    let inst = decode(&vec!(5,1,2)[..], 0).unwrap();
    assert_eq!(inst, Inst::JumpIfTrue(InParam::Position(1), InParam::Position(2)));

    let inst = decode(&vec!(105,1,2)[..], 0).unwrap();
    assert_eq!(inst, Inst::JumpIfTrue(InParam::Immediate(1), InParam::Position(2)));

    let inst = decode(&vec!(1105,1,2)[..], 0).unwrap();
    assert_eq!(inst, Inst::JumpIfTrue(InParam::Immediate(1), InParam::Immediate(2)));
}

#[test]
fn test_decode_jumpiffalse() {
    let inst = decode(&vec!(6,1,2)[..], 0).unwrap();
    assert_eq!(inst, Inst::JumpIfFalse(InParam::Position(1), InParam::Position(2)));

    let inst = decode(&vec!(106,1,2)[..], 0).unwrap();
    assert_eq!(inst, Inst::JumpIfFalse(InParam::Immediate(1), InParam::Position(2)));

    let inst = decode(&vec!(1106,1,2)[..], 0).unwrap();
    assert_eq!(inst, Inst::JumpIfFalse(InParam::Immediate(1), InParam::Immediate(2)));
}

#[test]
fn test_decode_lessthan() {
    let inst = decode(&vec!(7,1,2,3)[..], 0).unwrap();
    assert_eq!(inst, Inst::LessThan(InParam::Position(1), InParam::Position(2), OutParam::Position(3)));

    let inst = decode(&vec!(107,1,2,3)[..], 0).unwrap();
    assert_eq!(inst, Inst::LessThan(InParam::Immediate(1), InParam::Position(2), OutParam::Position(3)));

    let inst = decode(&vec!(1107,1,2,3)[..], 0).unwrap();
    assert_eq!(inst, Inst::LessThan(InParam::Immediate(1), InParam::Immediate(2), OutParam::Position(3)));
}

#[test]
fn test_decode_equal() {
    let inst = decode(&vec!(8,1,2,3)[..], 0).unwrap();
    assert_eq!(inst, Inst::Equal(InParam::Position(1), InParam::Position(2), OutParam::Position(3)));

    let inst = decode(&vec!(108,1,2,3)[..], 0).unwrap();
    assert_eq!(inst, Inst::Equal(InParam::Immediate(1), InParam::Position(2), OutParam::Position(3)));

    let inst = decode(&vec!(1108,1,2,3)[..], 0).unwrap();
    assert_eq!(inst, Inst::Equal(InParam::Immediate(1), InParam::Immediate(2), OutParam::Position(3)));
}

fn load(p: &[i32], param: &InParam) -> Result<i32, &'static str> {
    match param {
        InParam::Immediate(i) => Ok(*i),
        InParam::Position(i) => {
            if *i < 0 || *i >= p.len() as i32 {
                return Err("Bad load address");
            }
            Ok(p[*i as usize])
        }
    }
}

fn store(p: &mut[i32], param: &OutParam, value: i32) -> Result<(), &'static str> {
    match param {
        OutParam::Position(i) => {
            if *i < 0 || *i >= p.len() as i32 {
                return Err("Bad store address");
            }
            p[*i as usize] = value;
            Ok(())
        }
    }
}

pub fn run(p: &mut Vec<i32>, start: usize, input: Vec<i32>) -> Result<Vec<i32>, &'static str> {
    let mut pc: usize = start;
    let mut input = input;
    let mut output: Vec<i32> = Vec::new();

    loop {
        let inst = decode(&p[..], pc)?;
        pc = pc + inst.len();
        match &inst {
            Inst::Add(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                store(&mut p[..], &dst, p1 + p2)?;
                Ok(())
            },
            Inst::Mult(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                store(&mut p[..], &dst, p1 * p2)?;
                Ok(())
            },
            Inst::Input(dst) => {
                let value = input.remove(0);
                store(&mut p[..], &dst, value)?;
                Ok(())
            },
            Inst::Output(src) => {
                let p1 = load(&p[..], &src)?;
                output.push(p1);
                Ok(())
            },
            Inst::JumpIfTrue(cond, target) => {
                let cond = load(&p[..], &cond)?;
                if cond != 0 {
                    pc = load(&p[..], &target)? as usize;
                }
                Ok(())
            },
            Inst::JumpIfFalse(cond, target) => {
                let cond = load(&p[..], &cond)?;
                if cond == 0 {
                    pc = load(&p[..], &target)? as usize;
                }
                Ok(())
            },
            Inst::LessThan(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                if p1 < p2 {
                    store(&mut p[..], &dst, 1)?;
                } else {
                    store(&mut p[..], &dst, 0)?;
                }
                Ok(())
            }
            Inst::Equal(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                if p1 == p2 {
                    store(&mut p[..], &dst, 1)?;
                } else {
                    store(&mut p[..], &dst, 0)?;
                }
                Ok(())
            }
            Inst::Exit => break
        }?;
    }

    Ok(output)
}

pub fn read_from_string(s: &str) -> Vec<i32> {
    s.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

pub fn read_from_path(path: &str) -> std::io::Result<Vec<i32>> {
    let contents = std::fs::read_to_string(path)?;
    let numbers: Vec<i32> = read_from_string(&contents);
    Ok(numbers)
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn provided_case_1() {
    let mut p = vec!(1,9,10,3,2,3,11,0,99,30,40,50);
    run(&mut p, 0, vec!()).unwrap();

    assert_eq!(p, vec!(3500,9,10,70,2,3,11,0,99,30,40,50));
}

#[test]
fn provided_case_2() {
    let mut p = vec!(1,0,0,0,99);
    run(&mut p, 0, vec!()).unwrap();

    assert_eq!(p, vec!(2,0,0,0,99));
}

#[test]
fn run_with_io() {
    let mut p = vec!(3,1,99);
    let result = run(&mut p, 0, vec!(7)).unwrap();
    assert_eq!(p, vec!(3,7,99));
    assert_eq!(result, vec!());

    let mut p = vec!(3,5,104,10,4,11,99);
    let result = run(&mut p, 0, vec!(0)).unwrap();
    assert_eq!(p, vec!(3,5,104,10,4,0,99));
    assert_eq!(result, vec!(10,3));
}

#[test]
fn run_with_comparisons() {
    let mut p = vec!(3,9,8,9,10,9,4,9,99,-1,8);
    let result = run(&mut p, 0, vec!(7)).unwrap();
    assert_eq!(result, vec!(0));

    let mut p = vec!(3,9,8,9,10,9,4,9,99,-1,8);
    let result = run(&mut p, 0, vec!(8)).unwrap();
    assert_eq!(result, vec!(1));
}

}
