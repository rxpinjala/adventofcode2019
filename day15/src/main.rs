use std::collections::{BTreeMap, BTreeSet};
use std::env;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Self> for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_int(self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }

    fn from_int(i: u8) -> Self {
        match i {
            1 => Self::North,
            2 => Self::South,
            3 => Self::West,
            4 => Self::East,
            _ => panic!("Invalid direction"),
        }
    }

    // "p2 is ___ of p1"
    fn from_points(p1: Point, p2: Point) -> Self {
        let delta = (p2.x - p1.x, p2.y - p1.y);
        match delta {
            (1, 0) => Self::East,
            (-1, 0) => Self::West,
            (0, -1) => Self::North,
            (0, 1) => Self::South,
            _ => panic!("Bad directions"),
        }
    }

    fn random() -> Self {
        Self::from_int((rand::random::<u8>() % 4) + 1)
    }

    fn vector(self) -> Point {
        match self {
            Self::North => Point::new(0, -1),
            Self::South => Point::new(0, 1),
            Self::West => Point::new(-1, 0),
            Self::East => Point::new(1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    Moved,
    HitAWall,
    FoundTheThing,
}

impl Status {
    fn from_int(i: i64) -> Result<Self, String> {
        match i {
            0 => Ok(Self::HitAWall),
            1 => Ok(Self::Moved),
            2 => Ok(Self::FoundTheThing),
            _ => Err("Invalid status".to_string()),
        }
    }
}

struct Droid {
    computer: intcode::Computer,
    current_position: Point,
}

impl Droid {
    fn new(computer: intcode::Computer) -> Self {
        Self {
            computer,
            current_position: Point::new(0, 0),
        }
    }

    fn try_move(&mut self, direction: Direction) -> Result<(Status, Point), String> {
        assert!(self.computer.input.is_empty());
        self.computer.input.push_back(direction.to_int());

        loop {
            let step_result = self.computer.step()?;

            assert!(!step_result.input_needed);
            assert!(!step_result.done);
            if step_result.output_available {
                let output = self.computer.take_output();
                let status = Status::from_int(output[0])?;

                if status != Status::HitAWall {
                    self.current_position = self.current_position + direction.vector();
                }

                return match status {
                    Status::Moved => Ok((status, self.current_position)),
                    Status::HitAWall => Ok((status, self.current_position + direction.vector())),
                    Status::FoundTheThing => Ok((status, self.current_position)),
                };
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Wall,
    Clear,
    OxygenThing,
    Unknown,
}

struct Map {
    points: BTreeMap<Point, State>,
}

impl Map {
    fn new() -> Self {
        Map {
            points: BTreeMap::new(),
        }
    }

    fn get_point(&self, point: Point) -> State {
        *self.points.get(&point).unwrap_or(&State::Unknown)
    }

    fn print(&self, droid_position: Point) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for (pt, _) in self.points.iter() {
            min_x = std::cmp::min(pt.x, min_x);
            max_x = std::cmp::max(pt.x, max_x);
            min_y = std::cmp::min(pt.y, min_y);
            max_y = std::cmp::max(pt.y, max_y);
        }

        min_x -= 2;
        max_x += 3;
        min_y -= 2;
        max_y += 3;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let ch = if droid_position == Point::new(x, y) {
                    '@'
                } else if x == 0 && y == 0 {
                    'O'
                } else {
                    match self.get_point(Point::new(x, y)) {
                        State::Wall => '#',
                        State::Clear => ' ',
                        State::OxygenThing => 'X',
                        State::Unknown => '?',
                    }
                };

                print!("{}", ch);
            }
            println!();
        }
        println!();
    }

    fn update_from_move(&mut self, point: Point, status: Status) {
        match status {
            Status::HitAWall => {
                self.points.insert(point, State::Wall);
            }
            Status::Moved => {
                self.points.insert(point, State::Clear);
            }
            Status::FoundTheThing => {
                self.points.insert(point, State::OxygenThing);
            }
        }
    }

    fn find_possible_path(&self, start: Point, end: Point) -> Result<Vec<Point>, String> {
        let mut path_points: BTreeMap<Point, Point> = BTreeMap::new();
        path_points.insert(end, end);

        loop {
            let mut new_path_points: BTreeMap<Point, Point> = BTreeMap::new();
            let mut add_path_point_if_navigable = |point: Point, new_point: Point| {
                if self.get_point(new_point) != State::Wall && !path_points.contains_key(&new_point)
                {
                    new_path_points.insert(new_point, point);
                }
            };

            for (point, _) in path_points.iter() {
                add_path_point_if_navigable(*point, *point + Direction::North.vector());
                add_path_point_if_navigable(*point, *point + Direction::South.vector());
                add_path_point_if_navigable(*point, *point + Direction::East.vector());
                add_path_point_if_navigable(*point, *point + Direction::West.vector());
            }

            path_points.append(&mut new_path_points);

            if path_points.contains_key(&start) {
                break;
            }
        }

        let mut current_point = start;
        let mut result: Vec<Point> = Vec::new();
        while current_point != end {
            current_point = path_points[&current_point];
            result.push(current_point);
        }

        Ok(result)
    }
}

fn main() -> Result<(), String> {
    let input_path = &env::args().nth(1).unwrap();
    let computer = intcode::Computer::load_from_path(input_path)
        .map_err(|err| format!("Couldn't load input file: {}", err))?;

    let map = part1(computer.clone())?;
    part2(map, computer)?;
    Ok(())
}

fn part1(computer: intcode::Computer) -> Result<Map, String> {
    let mut droid = Droid::new(computer);
    let mut map = Map::new();
    let initial_position = Point::new(0, 0);
    map.points.insert(initial_position, State::Clear);
    let mut current_direction = Direction::South;
    let mut oxygen_thing_location: Option<Point> = None;

    // Find the thing
    loop {
        if map.get_point(droid.current_position + Direction::North.vector()) == State::Unknown {
            current_direction = Direction::North;
        } else if map.get_point(droid.current_position + Direction::South.vector())
            == State::Unknown
        {
            current_direction = Direction::South;
        } else if map.get_point(droid.current_position + Direction::East.vector()) == State::Unknown
        {
            current_direction = Direction::East;
        } else if map.get_point(droid.current_position + Direction::West.vector()) == State::Unknown
        {
            current_direction = Direction::West;
        }

        let (status, point) = droid.try_move(current_direction)?;
        map.update_from_move(point, status);

        if status == Status::HitAWall {
            current_direction = Direction::random();
        }

        if status == Status::FoundTheThing {
            oxygen_thing_location = Some(droid.current_position);
            break;
        }
    }

    // Find a path to the thing
    map.print(droid.current_position);
    let mut return_path: Vec<Point>;
    loop {
        // strategy:
        // find a path
        // find the first unknown tile in the path
        // move to that
        // compute a new path
        // break when there are no unknowns in the path

        return_path = map.find_possible_path(oxygen_thing_location.unwrap(), initial_position)?;

        let first_unknown_point = return_path
            .iter()
            .filter(|pt| map.get_point(**pt) == State::Unknown)
            .nth(0);
        if first_unknown_point.is_none() {
            break;
        }

        let test_path =
            map.find_possible_path(droid.current_position, *first_unknown_point.unwrap())?;
        for next_point in test_path.iter() {
            let direction = Direction::from_points(droid.current_position, *next_point);
            let (status, point) = droid.try_move(direction)?;
            map.update_from_move(point, status);
            if status == Status::HitAWall {
                break;
            }
        }

        map.print(droid.current_position);
    }

    map.print(droid.current_position);
    println!("Part 1: {}", return_path.len());
    Ok(map)
}

fn part2(map: Map, computer: intcode::Computer) -> Result<(), String> {
    let mut filled_points: BTreeMap<Point, u32> = BTreeMap::new();
    let droid = Droid::new(computer);

    let mut generation: u32 = 0;
    let oxygen_thing_location = *map.points.iter()
        .filter(|(k, v)| **v == State::OxygenThing)
        .nth(0)
        .ok_or("couldn't find oxygen thing from part 1".to_string())?.0;    
    filled_points.insert(oxygen_thing_location, generation);

    loop {
        let prev_count = filled_points.len();
        let prev_generation = generation;
        generation += 1;

        let mut new_filled_points: BTreeMap<Point, u32> = BTreeMap::new();
        for (k, v) in filled_points.iter() {
            if *v == prev_generation {
                let mut add_point_if_needed = |point: Point| {
                    if map.get_point(point) == State::Clear && !filled_points.contains_key(&point) {
                        new_filled_points.insert(point, generation);
                    }
                };
                add_point_if_needed(*k + Direction::North.vector());
                add_point_if_needed(*k + Direction::South.vector());
                add_point_if_needed(*k + Direction::East.vector());
                add_point_if_needed(*k + Direction::West.vector());
            }
        }

        filled_points.append(&mut new_filled_points);

        if filled_points.len() == prev_count {
            break;
        }
    }

    println!("Part 2: {}", generation - 1);

    Ok(())
}
