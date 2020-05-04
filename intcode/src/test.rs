
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
    // single input
    let mut p = vec!(3,1,99);
    let result = run(&mut p, 0, vec!(7)).unwrap();
    assert_eq!(p, vec!(3,7,99));
    assert_eq!(result, vec!());

    // input + output
    let mut p = vec!(3,5,104,10,4,11,99);
    let result = run(&mut p, 0, vec!(0)).unwrap();
    assert_eq!(p, vec!(3,5,104,10,4,0,99));
    assert_eq!(result, vec!(10,3));
}

#[test]
fn run_with_multiple_io() {
    // multiple inputs, multiple outputs
    let mut p = vec!(3,13,3,14,3,15,4,15,4,14,4,13,99,0,0,0);
    let result = run(&mut p, 0, vec!(22,33,44)).unwrap();
    assert_eq!(p, vec!(3,13,3,14,3,15,4,15,4,14,4,13,99,22,33,44));
    assert_eq!(result, vec!(44,33,22));
}

#[test]
fn run_with_branches() {
    let mut p = vec!(
        1001,12,-1,12, // [12] = ADD [12],-1
        108,0,12,13,   // [13] = EQ 0,[12]
        1006,13,0,     // IF ![13] GOTO 0
        99,            // EXIT
        10,0);         // 12, 13
    let _ = run(&mut p, 0, vec!()).unwrap();
    assert_eq!(p, vec!(1001,12,-1,12,108,0,12,13,1006,13,0,99,0,1));
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

fn encode_opcode_ssd(op: i32, p1: InParam, p2: InParam, dst: OutParam) -> [i32;4] {
    let mut op = op;
    let mut result: [i32; 4] = [0; 4];

    match p1 {
        InParam::Position(i) => {
            result[1] = i;
        }
        InParam::Immediate(i) => {
            result[1] = i;
            op = op + 100;
        }
    }

    match p2 {
        InParam::Position(i) => {
            result[2] = i;
        }
        InParam::Immediate(i) => {
            result[2] = i;
            op = op + 1000;
        }
    }

    match dst {
        OutParam::Position(i) => {
            result[3] = i;
        }
    }

    result[0] = op;
    result
}

fn encode_opcode_ss(op: i32, p1: InParam, p2: InParam) -> [i32;3] {
    let mut op = op;
    let mut result: [i32; 3] = [0; 3];

    match p1 {
        InParam::Position(i) => {
            result[1] = i;
        }
        InParam::Immediate(i) => {
            result[1] = i;
            op = op + 100;
        }
    }

    match p2 {
        InParam::Position(i) => {
            result[2] = i;
        }
        InParam::Immediate(i) => {
            result[2] = i;
            op = op + 1000;
        }
    }

    result[0] = op;
    result
}

fn encode_opcode_s(op: i32, src: InParam) -> [i32;2] {
    let mut op = op;
    let mut result: [i32; 2] = [0; 2];

    match src {
        InParam::Position(i) => {
            result[1] = i;
        }
        InParam::Immediate(i) => {
            result[1] = i;
            op = op + 100;
        }
    }

    result[0] = op;
    result
}

fn encode_opcode_d(op: i32, dst: OutParam) -> [i32;2] {
    let mut result: [i32; 2] = [0; 2];
    match dst {
        OutParam::Position(i) => {
            result[1] = i;
        }
    }

    result[0] = op;
    result
}

fn encode(insts: &[Inst]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for inst in insts.iter() {
        match inst {
            Inst::Add(src1, src2, dst) => {
                result.extend_from_slice(&encode_opcode_ssd(1, *src1, *src2, *dst));
            }
            Inst::Mult(src1, src2, dst) => {
                result.extend_from_slice(&encode_opcode_ssd(2, *src1, *src2, *dst));
            }
            Inst::Input(dst) => {
                result.extend_from_slice(&encode_opcode_d(3, *dst));
            }
            Inst::Output(src) => {
                result.extend_from_slice(&encode_opcode_s(4, *src));
            }
            Inst::JumpIfTrue(src1, src2) => {
                result.extend_from_slice(&encode_opcode_ss(5, *src1, *src2));
            }
            Inst::JumpIfFalse(src1, src2) => {
                result.extend_from_slice(&encode_opcode_ss(6, *src1, *src2));
            }
            Inst::LessThan(src1, src2, dst) => {
                result.extend_from_slice(&encode_opcode_ssd(7, *src1, *src2, *dst));
            }
            Inst::Equal(src1, src2, dst) => {
                result.extend_from_slice(&encode_opcode_ssd(8, *src1, *src2, *dst));
            }
            Inst::Exit => {
                result.push(99);
            }
        }
    }

    result
}

#[test]
fn test_encode() {
    assert_eq!(encode(&[
            Inst::Add(InParam::Immediate(3), InParam::Immediate(4), OutParam::Position(1))
        ]),
        vec!(1101, 3, 4, 1)
    )
}
