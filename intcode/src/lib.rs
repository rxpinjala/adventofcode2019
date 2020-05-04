
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InParam {
    Position(i32),
    Immediate(i32),
}

impl fmt::Display for InParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InParam::Position(i) => 
                write!(f, "[{}]", i),
            InParam::Immediate(i) => 
                write!(f, "{}", i)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutParam {
    Position(i32),
}

impl fmt::Display for OutParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutParam::Position(i) => 
                write!(f, "[{}]", i)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub type PC = usize;

#[derive(Debug)]
pub enum RunResult {
    Done,
    WaitingForInput(PC),
    Output(PC, i32)
}

pub fn run_step(p: &mut [i32], start: PC, input: Option<i32>) -> Result<RunResult, &'static str> {
    let mut pc: PC = start;
    let mut input = input;

    loop {
        let inst = decode(&p[..], pc)?;
        let mut next_pc = pc + inst.len();
        match &inst {
            Inst::Add(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                println!("{}: {} = ADD {} ({}) {} ({})", pc, dst, src1, p1, src2, p2);
                store(&mut p[..], &dst, p1 + p2)?;
            },
            Inst::Mult(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                println!("{}: {} = MULT {} ({}) {} ({})", pc, dst, src1, p1, src2, p2);
                store(&mut p[..], &dst, p1 * p2)?;
            },
            Inst::Input(dst) => {
                if let Some(input_value) = input {
                    println!("{}: {} = INPUT {}", pc, dst, input_value);
                    store(&mut p[..], &dst, input_value)?;
                    input = Option::None;
                } else {
                    return Ok(RunResult::WaitingForInput(pc));
                }
            },
            Inst::Output(src) => {
                let p1 = load(&p[..], &src)?;
                println!("{}: OUTPUT {} ({})", pc, src, p1);
                return Ok(RunResult::Output(next_pc, p1));
            },
            Inst::JumpIfTrue(cond, target) => {
                let cond_value = load(&p[..], &cond)?;
                let target_value = load(&p[..], &target)? as usize;
                println!("{}: IF {} ({}) GOTO {} ({})", pc, cond, cond_value, target, target_value);
                if cond_value != 0 {
                    next_pc = target_value;
                }
            },
            Inst::JumpIfFalse(cond, target) => {
                let cond_value = load(&p[..], &cond)?;
                let target_value = load(&p[..], &target)? as usize;
                println!("{}: IF NOT {} ({}) GOTO {} ({})", pc, cond, cond_value, target, target_value);
                if cond_value == 0 {
                    next_pc = target_value;
                }
            },
            Inst::LessThan(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                println!("{}: {} = {} ({}) < {} ({})", pc, dst, src1, p1, src2, p2);
                if p1 < p2 {
                    store(&mut p[..], &dst, 1)?;
                } else {
                    store(&mut p[..], &dst, 0)?;
                }
            }
            Inst::Equal(src1, src2, dst) => {
                let p1 = load(&p[..], &src1)?;
                let p2 = load(&p[..], &src2)?;
                println!("{}: {} = {} ({}) == {} ({})", pc, dst, src1, p1, src2, p2);
                if p1 == p2 {
                    store(&mut p[..], &dst, 1)?;
                } else {
                    store(&mut p[..], &dst, 0)?;
                }
            }
            Inst::Exit => {
                println!("{}: EXIT", pc);
                return Ok(RunResult::Done);
            }
        };
        pc = next_pc;
    }
}

pub fn run(p: &mut Vec<i32>, start: usize, input: Vec<i32>) -> Result<Vec<i32>, &'static str> {
    let mut pc: usize = start;
    let mut input = input;
    input.reverse();
    let mut next_input = input.pop();
    let mut output: Vec<i32> = Vec::new();

    loop {
        let result = run_step(&mut p[..], pc, next_input)?;
        match result {
            RunResult::Done => return Ok(output),
            RunResult::Output(pc_new, val) => {
                output.push(val);
                pc = pc_new;
            },
            RunResult::WaitingForInput(pc_new) => {
                next_input = input.pop();
                pc = pc_new;
            }
        };
    }
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
mod test;
