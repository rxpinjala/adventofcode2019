use std::collections::BTreeMap;
use std::env;

type Point = (i32, i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    Black,
    White,
}

#[derive(Debug)]
struct Grid {
    current_position: Point,
    current_direction: i8, // up: 0, right: 1, down: 2, left: 3
    painted_points: BTreeMap<Point, Color>,
}

impl Grid {
    fn new() -> Self {
        Grid{current_position: (0, 0), current_direction: 0, painted_points: BTreeMap::new()}
    }

    fn color_at(&self, pt: Point) -> Color {
        match self.painted_points.get(&pt) {
            Some(color) => *color,
            None => Color::Black,
        }
    }

    fn paint(&mut self, pt: Point, color: Color) {
        self.painted_points.insert(pt, color);
    }

    fn move_by_delta(&mut self, delta: Point) {
        self.current_position = (
            self.current_position.0 + delta.0,
            self.current_position.1 + delta.1,
        );
    }

    fn move_in_current_direction(&mut self) {
        match self.current_direction {
            0 => self.move_by_delta((0, 1)),
            1 => self.move_by_delta((1, 0)),
            2 => self.move_by_delta((0, -1)),
            3 => self.move_by_delta((-1, 0)),
            _ => panic!()
        }
    }

    fn turn_left_and_move(&mut self) {
        self.current_direction = (self.current_direction + 3) % 4;
        self.move_in_current_direction();
    }

    fn turn_right_and_move(&mut self) {
        self.current_direction = (self.current_direction + 1) % 4;
        self.move_in_current_direction();
    }
}

static PART2: bool = true;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let mut c = intcode::Computer::load_from_path(&input_path).unwrap();
    let mut grid = Grid::new();

    if PART2 {
        grid.paint((0, 0), Color::White);
    }

    loop {
        let step_result = c.step().unwrap();

        if step_result.done {
            break;
        }

        if step_result.input_needed {
            c.send_input(match grid.color_at(grid.current_position) {
                Color::White => 1,
                Color::Black => 0,
            });
        }

        if step_result.output_available && c.output.len() == 2 {
            let color = c.output.pop_front().unwrap();
            let direction = c.output.pop_front().unwrap();

            match color {
                0 => grid.paint(grid.current_position, Color::Black),
                1 => grid.paint(grid.current_position, Color::White),
                _ => panic!("Unexpected color"),
            }

            match direction {
                0 => grid.turn_left_and_move(),
                1 => grid.turn_right_and_move(),
                _ => panic!("Unexpected direction"),
            }
        }
    }

    if PART2 {
        dbg!(&grid.painted_points);
        for y in -8..8 {
            for x in 0..50 {
                let point = (x, y);
                print!("{}", match grid.color_at(point) {
                    Color::Black => '#',
                    Color::White => ' ',
                });
            }
            println!();
        }
    } else {
        println!("Part 1: {}", grid.painted_points.len());
    }

}
