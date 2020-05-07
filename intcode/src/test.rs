
use super::*;

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

#[test]
fn provided_case_1() {
    let mut c = Computer::new(vec!(1,9,10,3,2,3,11,0,99,30,40,50));
    c.run().unwrap();

    assert_eq!(c.mem, vec!(3500,9,10,70,2,3,11,0,99,30,40,50));
}

#[test]
fn provided_case_2() {
    let mut c = Computer::new(vec!(1,0,0,0,99));
    c.run().unwrap();

    assert_eq!(c.mem, vec!(2,0,0,0,99));
}

#[test]
fn run_with_io() {
    // single input
    let mut computer = Computer::new(vec!(3,1,99));
    computer.input.push_back(7);
    computer.run().unwrap();

    assert_eq!(computer.mem, vec!(3,7,99));
    assert_eq!(computer.output.len(), 0);

    // input + output
    let mut computer = Computer::new(vec!(3,5,104,10,4,11,99));
    computer.input.push_back(0);
    computer.run().unwrap();

    assert_eq!(computer.mem, vec!(3,5,104,10,4,0,99));
    assert_eq!(computer.take_output(), vec!(10,3));
}

#[test]
fn run_with_multiple_io() {
    // multiple inputs, multiple outputs
    let mut c = Computer::new(vec!(3,13,3,14,3,15,4,15,4,14,4,13,99,0,0,0));
    c.input.push_back(22);
    c.input.push_back(33);
    c.input.push_back(44);
    c.run().unwrap();
    assert_eq!(c.mem, vec!(3,13,3,14,3,15,4,15,4,14,4,13,99,22,33,44));
    assert_eq!(c.take_output(), vec!(44,33,22));
}

#[test]
fn run_with_branches() {
    let mut c = Computer::new(vec!(
        1001,12,-1,12, // [12] = ADD [12],-1
        108,0,12,13,   // [13] = EQ 0,[12]
        1006,13,0,     // IF ![13] GOTO 0
        99,            // EXIT
        10,0));        // 12, 13
    c.run().unwrap();
    assert_eq!(c.mem, vec!(1001,12,-1,12,108,0,12,13,1006,13,0,99,0,1));
}

#[test]
fn run_with_comparisons() {
    let mut c = Computer::new(vec!(3,9,8,9,10,9,4,9,99,-1,8));
    c.input.push_back(7);
    c.run().unwrap();
    assert_eq!(c.take_output(), vec!(0));

    let mut c = Computer::new(vec!(3,9,8,9,10,9,4,9,99,-1,8));
    c.input.push_back(8);
    c.run().unwrap();
    assert_eq!(c.take_output(), vec!(1));
}

#[test]
fn day9_case_1() {
    // takes no input and produces a copy of itself as output
    let input = vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99);
    let mut c = Computer::new(input.clone());
    c.run().unwrap();
    assert_eq!(input, c.take_output());
}

#[test]
fn day9_case_2() {
    // should output a 16-digit number
    let mut c = Computer::new(vec!(1102,34915192,34915192,7,4,7,99,0));
    c.run().unwrap();

    let output_as_string = format!("{}", c.take_output()[0]);
    assert_eq!(output_as_string.len(), 16);
}

#[test]
fn day9_case_3() {
    let mut c = Computer::new(vec!(104,1125899906842624,99));
    c.run().unwrap();
    assert_eq!(c.take_output()[0], 1125899906842624);
}