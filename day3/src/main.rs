use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;

static PART2: bool = true;

fn main() {
    let input = read_input().unwrap();

    let trace_0 = trace_wire(&input.0[..]);
    let trace_1 = trace_wire(&input.1[..]);

    if PART2 {
        let intersections: BTreeSet<Position> = find_intersections(&trace_0, &trace_1).into_iter().collect();
        dbg!(trace_0.len());
        dbg!(trace_1.len());

        let mut best: i32 = 1000000;

        for (i0, p0) in trace_0.iter().enumerate() {
            if !intersections.contains(&p0) || i0 == 0 {
                continue
            }
            for (i1, p1) in trace_1.iter().enumerate() {
                if p0 == p1 {
                    best = std::cmp::min((i0 + i1) as i32, best);
                }
            }
        }

        println!("Result: {}", best);
    } else {
        let mut intersections = find_intersections(&trace_0, &trace_1);
        intersections.sort_by(|p1, p2| p1.distance_from_origin().cmp(&p2.distance_from_origin()));
        intersections.remove(0); // always (0, 0)
        println!("Result: {}", intersections[0].distance_from_origin());
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Clone, Debug, PartialEq)]
struct Move {
    direction: Direction,
    length: i32
}

impl Move {
    fn new(dir: Direction, len: i32) -> Move {
        Move { direction: dir, length: len }
    }
}

fn parse_move(data: &str) -> Result<Move, &'static str> {
    let dir = match data.chars().nth(0) {
        Some('L') => Direction::Left,
        Some('R') => Direction::Right,
        Some('U') => Direction::Up,
        Some('D') => Direction::Down,
        _ => Err("Failed to parse move")?
    };

    let len = data[1..].parse::<i32>().map_err(|_| "Failed to parse int")?;

    Ok(Move::new(dir, len))
}

#[test]
fn test_parse_move() {
    assert_eq!(parse_move("L10").unwrap(), Move::new(Direction::Left, 10));
    assert_eq!(parse_move("R10").unwrap(), Move::new(Direction::Right, 10));
    assert_eq!(parse_move("U10").unwrap(), Move::new(Direction::Up, 10));
    assert_eq!(parse_move("D10").unwrap(), Move::new(Direction::Down, 10));
}

fn parse_line(data: &str) -> Result<Vec<Move>, &'static str> {
    data.split(",").map(parse_move).collect::<Result<Vec<Move>, &'static str>>()
}

#[test]
fn test_parse_line() -> Result<(), &'static str> {
    let expected_result = vec!(
        Move::new(Direction::Left, 10), 
        Move::new(Direction::Right, 5), 
        Move::new(Direction::Up, 7), 
        Move::new(Direction::Down, 111));
    assert_eq!(expected_result, parse_line("L10,R5,U7,D111")?);
    Ok(())
}

fn read_input() -> io::Result<(Vec<Move>, Vec<Move>)> {
    let input_path: &String = &env::args().nth(1).unwrap();
    let input_data = fs::read_to_string(input_path)?;
    let input_lines = input_data.lines().map(parse_line).collect::<Result<Vec<Vec<Move>>, &'static str>>().unwrap();

    Ok((input_lines[0].clone(), input_lines[1].clone()))
}

fn trace_wire(moves: &[Move]) -> Vec<Position> {
    let mut current_position = Position::new(0, 0);

    let mut result: Vec<Position> = Vec::new();
    result.push(current_position.clone());

    for m in moves.iter() {
        for _ in 0..m.length {
            match m.direction {
                Direction::Up => current_position.y = current_position.y + 1,
                Direction::Down => current_position.y = current_position.y - 1,
                Direction::Left => current_position.x = current_position.x - 1,
                Direction::Right => current_position.x = current_position.x + 1,
            };

            result.push(current_position.clone());
        }
    }

    result
}

#[test]
fn test_trace_wire() {
    let input = vec!(Move::new(Direction::Right, 3));
    let result = trace_wire(&input[..]);
    let mut expected_result = Vec::new();
    expected_result.push(Position::new(0, 0));
    expected_result.push(Position::new(1, 0));
    expected_result.push(Position::new(2, 0));
    expected_result.push(Position::new(3, 0));
    assert_eq!(result, expected_result);

    let input = vec!(Move::new(Direction::Right, 3), Move::new(Direction::Up, 1));
    let result = trace_wire(&input[..]);
    let mut expected_result = Vec::new();
    expected_result.push(Position::new(0, 0));
    expected_result.push(Position::new(1, 0));
    expected_result.push(Position::new(2, 0));
    expected_result.push(Position::new(3, 0));
    expected_result.push(Position::new(3, 1));
    assert_eq!(result, expected_result);

}

fn find_intersections(trace0: &Vec<Position>, trace1: &Vec<Position>) -> Vec<Position> {
    let trace0: BTreeSet<Position> = trace0.iter().cloned().collect();
    let trace1: BTreeSet<Position> = trace1.iter().cloned().collect();

    trace0.intersection(&trace1).cloned().collect()
}