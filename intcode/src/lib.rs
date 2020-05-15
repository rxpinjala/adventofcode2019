
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InParam {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

impl InParam {
    fn with_mode(mode: i8, value: i64) -> Result<Self, &'static str> {
        match mode {
            0 => Ok(Self::Position(value)),
            1 => Ok(Self::Immediate(value)),
            2 => Ok(Self::Relative(value)),
            _ => Err("Unrecognized mode"),
        }
    }
}

impl fmt::Display for InParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InParam::Position(i) => 
                write!(f, "pos:{}", i),
            InParam::Immediate(i) => 
                write!(f, "imm:{}", i),
            InParam::Relative(i) =>
                write!(f, "rel:{}", i),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OutParam {
    Position(i64),
    Relative(i64),
}

impl OutParam {
    fn with_mode(mode: i8, value: i64) -> Result<Self, &'static str> {
        match mode {
            0 => Ok(Self::Position(value)),
            2 => Ok(Self::Relative(value)),
            _ => Err("Unrecognized mode"),
        }
    }
}

impl fmt::Display for OutParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutParam::Position(i) => 
                write!(f, "pos:{}", i),
            OutParam::Relative(i) =>
                write!(f, "rel:{}", i),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Inst {
    Add(InParam, InParam, OutParam),
    Mult(InParam, InParam, OutParam),
    Input(OutParam),
    Output(InParam),
    JumpIfTrue(InParam, InParam),
    JumpIfFalse(InParam, InParam),
    LessThan(InParam, InParam, OutParam),
    Equal(InParam, InParam, OutParam),
    AdjustBase(InParam),
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
            Inst::AdjustBase(_) => 2
        }
    }
}

fn modes(opcode: i64) -> [i8; 4] {
    let mut result = [0 as i8,0,0,0];
    let mut i = 0;
    let mut m = opcode / 100;

    while m > 0 {
        result[i] = (m % 10) as i8;
        m /= 10;
        i += 1;
    }

    result
}

fn decode(p: &[i64], pc: usize) -> Result<Inst, &'static str> {
    if pc >= p.len() {
        return Err("Bad PC");
    }

    let opcode = p[pc] % 100;
    let modes = modes(p[pc]);
    match opcode {
        1 => { // add
            let inst = p.get(pc..pc + 4).ok_or("Bad add instruction")?;
            Ok(Inst::Add(
                InParam::with_mode(modes[0], inst[1])?,
                InParam::with_mode(modes[1], inst[2])?,
                OutParam::with_mode(modes[2], inst[3])?,
            ))
        }

        2 => { // multiply
            let inst = p.get(pc..pc + 4).ok_or("Bad mul instruction")?;
            Ok(Inst::Mult(
                InParam::with_mode(modes[0], inst[1])?,
                InParam::with_mode(modes[1], inst[2])?,
                OutParam::with_mode(modes[2], inst[3])?,
            ))
        }

        3 => { // input
            let inst = p.get(pc..pc + 2).ok_or("Bad input instruction")?;
            Ok(Inst::Input(OutParam::with_mode(modes[0], inst[1])?))
        }

        4 => { // output
            let inst = p.get(pc..pc + 2).ok_or("Bad output instruction")?;
            Ok(Inst::Output(InParam::with_mode(modes[0], inst[1])?))
        }

        5 => { // jump if true
            let inst = p.get(pc..pc + 3).ok_or("Bad jump-if-true instruction")?;
            Ok(Inst::JumpIfTrue(
                InParam::with_mode(modes[0], inst[1])?,
                InParam::with_mode(modes[1], inst[2])?
            ))
        }

        6 => { // jump if false
            let inst = p.get(pc..pc + 3).ok_or("Bad jump-if-false instruction")?;
            Ok(Inst::JumpIfFalse(
                InParam::with_mode(modes[0], inst[1])?,
                InParam::with_mode(modes[1], inst[2])?
            ))
        }

        7 => { // less than
            let inst = p.get(pc..pc + 4).ok_or("Bad less-than instruction")?;
            Ok(Inst::LessThan(
                InParam::with_mode(modes[0], inst[1])?,
                InParam::with_mode(modes[1], inst[2])?,
                OutParam::with_mode(modes[2], inst[3])?,
            ))
        }

        8 => { // equal
            let inst = p.get(pc..pc + 4).ok_or("Bad equal instruction")?;
            Ok(Inst::Equal(
                InParam::with_mode(modes[0], inst[1])?,
                InParam::with_mode(modes[1], inst[2])?,
                OutParam::with_mode(modes[2], inst[3])?,
            ))
        }

        9 => { // adjust relative base
            let inst = p.get(pc..pc + 2).ok_or("Bad adjust instruction")?;
            Ok(Inst::AdjustBase(
                InParam::with_mode(modes[0], inst[1])?
            ))
        }

        99 => Ok(Inst::Exit),

        _ => Err("Bad instruction")
    }
}

pub fn read_from_string(s: &str) -> Vec<i64> {
    s.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect()
}

pub fn read_from_path(path: &str) -> std::io::Result<Vec<i64>> {
    let contents = std::fs::read_to_string(path)?;
    let numbers: Vec<i64> = read_from_string(&contents);
    Ok(numbers)
}

pub struct StepResult {
    pub done: bool,
    pub input_needed: bool,
    pub output_available: bool,
}

impl StepResult {
    fn ok(c: &Computer) -> Self {
        StepResult{ done: false, input_needed: false, output_available: !c.output.is_empty() }
    }

    fn done(c: &Computer) -> Self {
        StepResult{ done: true, input_needed: false, output_available: !c.output.is_empty() }
    }

    fn input_needed(c: &Computer) -> Self {
        StepResult{ done: false, input_needed: true, output_available: !c.output.is_empty() }
    }
}

#[derive(Clone, Debug)]
pub struct Computer {
    pub mem: Vec<i64>,
    pub pc: usize,
    pub relative_base: i64,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,

    pub enable_tracing: bool,
}

impl Computer {
    pub fn new(mem: Vec<i64>) -> Self {
        Computer{ mem, pc: 0, relative_base: 0, input: VecDeque::new(), output: VecDeque::new(), enable_tracing: false }
    }

    pub fn load_from_string(s: &str) -> Self {
        let mem = s.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        Computer{ mem, pc: 0, relative_base: 0, input: VecDeque::new(), output: VecDeque::new(), enable_tracing: false }
    }

    pub fn load_from_path(path: &str) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Ok(Self::load_from_string(&contents))
    }
    
    fn trace(&self, s: &str) {
        if self.enable_tracing {
            println!("{}", s);
        }
    }

    pub fn step(&mut self) -> Result<StepResult, &'static str> {
        let inst = decode(&self.mem[..], self.pc)?;
        let mut next_pc = self.pc + inst.len();

        match &inst {
            Inst::Add(src1, src2, dst) => {
                let p1 = self.load(&src1)?;
                let p2 = self.load(&src2)?;
                self.trace(&format!("{}: {} = ADD {} ({}) {} ({})", self.pc, dst, src1, p1, src2, p2));
                self.store(&dst, p1 + p2)?;
            },
            Inst::Mult(src1, src2, dst) => {
                let p1 = self.load(&src1)?;
                let p2 = self.load(&src2)?;
                self.trace(&format!("{}: {} = MULT {} ({}) {} ({})", self.pc, dst, src1, p1, src2, p2));
                self.store(&dst, p1 * p2)?;
            },
            Inst::Input(dst) => {
                if let Some(input_value) = self.input.pop_front() {
                    self.trace(&format!("{}: {} = INPUT {}", self.pc, dst, input_value));
                    self.store(&dst, input_value)?;
                } else {
                    return Ok(StepResult::input_needed(&self));
                }
            },
            Inst::Output(src) => {
                let p1 = self.load(&src)?;
                self.trace(&format!("{}: OUTPUT {} ({})", self.pc, src, p1));
                self.output.push_back(p1);
            },
            Inst::JumpIfTrue(cond, target) => {
                let cond_value = self.load(&cond)?;
                let target_value = self.load(&target)? as usize;
                self.trace(&format!("{}: IF {} ({}) GOTO {} ({})", self.pc, cond, cond_value, target, target_value));
                if cond_value != 0 {
                    next_pc = target_value;
                }
            },
            Inst::JumpIfFalse(cond, target) => {
                let cond_value = self.load(&cond)?;
                let target_value = self.load(&target)? as usize;
                self.trace(&format!("{}: IF NOT {} ({}) GOTO {} ({})", self.pc, cond, cond_value, target, target_value));
                if cond_value == 0 {
                    next_pc = target_value;
                }
            },
            Inst::LessThan(src1, src2, dst) => {
                let p1 = self.load(&src1)?;
                let p2 = self.load(&src2)?;
                self.trace(&format!("{}: {} = {} ({}) < {} ({})", self.pc, dst, src1, p1, src2, p2));
                if p1 < p2 {
                    self.store(&dst, 1)?;
                } else {
                    self.store(&dst, 0)?;
                }
            }
            Inst::Equal(src1, src2, dst) => {
                let p1 = self.load(&src1)?;
                let p2 = self.load(&src2)?;
                self.trace(&format!("{}: {} = {} ({}) == {} ({})", self.pc, dst, src1, p1, src2, p2));
                if p1 == p2 {
                    self.store(&dst, 1)?;
                } else {
                    self.store(&dst, 0)?;
                }
            }
            Inst::AdjustBase(src) => {
                let p1 = self.load(&src)?;
                self.relative_base += p1;
                self.trace(&format!("{}: ADJUST BASE BY {} ({}), NOW {}", self.pc, src, p1, self.relative_base));
            }
            Inst::Exit => {
                self.trace(&format!("{}: EXIT", self.pc));
                return Ok(StepResult::done(&self));
            }
        };
        self.pc = next_pc;

        Ok(StepResult::ok(&self))
    }
    
    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let result = self.step()?;
            if result.done {
                return Ok(());
            }
        }
    }

    fn load(&self, param: &InParam) -> Result<i64, &'static str> {
        match param {
            InParam::Immediate(i) => Ok(*i),
            InParam::Position(i) => {
                if *i < 0 {
                    return Err("Bad load address");
                }
                if *i >= self.mem.len() as i64 {
                    return Ok(0);
                }
                Ok(self.mem[*i as usize])
            },
            InParam::Relative(i) => {
                let i = i + self.relative_base;
                if i < 0 {
                    return Err("Bad rel load address");
                }
                if i >= self.mem.len() as i64 {
                    return Ok(0);
                }
                Ok(self.mem[i as usize])
            }
        }
    }
    
    fn store(&mut self, param: &OutParam, value: i64) -> Result<(), &'static str> {
        match param {
            OutParam::Position(i) => {
                if *i < 0 {
                    return Err("Bad store address");
                }
                if *i >= self.mem.len() as i64 {
                    self.mem.resize(*i as usize + 1, 0);
                }
                self.mem[*i as usize] = value;
                Ok(())
            },
            OutParam::Relative(i) => {
                let i = i + self.relative_base;
                if i < 0 {
                    return Err("Bad rel store address");
                }
                if i >= self.mem.len() as i64 {
                    self.mem.resize(i as usize + 1, 0);
                }
                self.mem[i as usize] = value;
                Ok(())
            }
        }
    }
    
    pub fn send_input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn take_output(&mut self) -> Vec<i64> {
        self.output.drain(..).collect()
    }
}

#[cfg(test)]
mod test;
