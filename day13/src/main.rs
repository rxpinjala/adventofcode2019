use std::collections::BTreeMap;
use std::env;

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();

    let mut c = intcode::Computer::load_from_path(input_path).unwrap();
    c.run().unwrap();
    let output = c.take_output();
    let tiles: Result<Vec<Tile>, String> = output.chunks(3).map(Tile::from_ints).collect();
    let tiles = tiles.unwrap();
    let part1_count = tiles.iter().filter(|t| t.tile_type == TileType::Block).count();
    println!("Part 1: {}", part1_count);

    let max_x = tiles.iter().map(|t| t.x).max().unwrap();
    let max_y = tiles.iter().map(|t| t.y).max().unwrap();

    dbg!(max_x);
    dbg!(max_y);

    let mut game = Game::new(intcode::Computer::load_from_path(input_path).unwrap());
    game.computer.mem[0] = 2;
    game.run();
}

#[derive(Debug, Eq, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    HorizPaddle,
    Ball,
}

impl TileType {
    fn from_int(i: i64) -> Result<Self, String> {
        match i {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::HorizPaddle),
            4 => Ok(Self::Ball),
            _ => Err("Invalid tile type".to_string())
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Wall => '+',
            Self::Block => '#',
            Self::HorizPaddle => '_',
            Self::Ball => '*',
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Tile {
    x: i64,
    y: i64,
    tile_type: TileType,
}

impl Tile {
    fn from_ints(ints: &[i64]) -> Result<Self, String> {
        if ints.len() != 3 {
            return Err(format!("Invalid tile data: {:?}", ints));
        }

        Ok(Self {
            x: ints[0],
            y: ints[1],
            tile_type: TileType::from_int(ints[2])?
        })
    }
}

struct Game {
    computer: intcode::Computer,
    tiles: BTreeMap<(i32, i32), TileType>,
    score: i32,
}

impl Game {
    fn new(computer: intcode::Computer) -> Self {
        Self {
            computer,
            tiles: BTreeMap::new(),
            score: 0,
        }
    }

    fn print_tiles(&self) {
        const MAX_X: i32 = 40;
        const MAX_Y: i32 = 25;

        for y in 0..MAX_Y {
            for x in 0..MAX_X {
                print!("{}", match self.tiles.get(&(x, y)) {
                    Some(tile_type) => {
                        tile_type.to_char()
                    },
                    None => {
                        ' '
                    },
                });
            }
            println!();
        }
    }

    fn find_tile(&self, target_tile_type: TileType) -> Option<(i32, i32)> {
        for (pos, tile_type) in self.tiles.iter() {
            if *tile_type == target_tile_type {
                return Some(*pos);
            }
        }

        None
    }

    fn run(&mut self) {
        loop {
            let step_result = self.computer.step().unwrap();
            if step_result.input_needed {
                println!("Score: {}", self.score);
                //self.print_tiles();

                let ball_pos = self.find_tile(TileType::Ball).unwrap();
                let paddle_pos = self.find_tile(TileType::HorizPaddle).unwrap();

                let input = if ball_pos.0 < paddle_pos.0 {
                    -1
                } else if ball_pos.0 > paddle_pos.0 {
                    1
                } else {
                    0
                };

                self.computer.send_input(input);
            }

            if step_result.output_available && self.computer.output.len() >= 3 {
                let x = self.computer.output.pop_front().unwrap() as i32;
                let y = self.computer.output.pop_front().unwrap() as i32;
                if x == -1 && y == 0 {
                    self.score = self.computer.output.pop_front().unwrap() as i32;
                } else {
                    let tile_type = TileType::from_int(self.computer.output.pop_front().unwrap()).unwrap();
                    self.tiles.insert((x, y), tile_type);
                }
            }

            if step_result.done {
                println!("Score: {}", self.score);
                break;
            }
        }
    
    
    }
}
