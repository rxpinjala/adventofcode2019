
use std::collections::BTreeSet;
use std::env;
use std::fs;

struct Map {
    dx: usize,
    dy: usize,
    data: Vec<Vec<bool>>
}

fn load_line(s: &str) -> Result<Vec<bool>, &'static str> {
    s.trim()
        .chars()
        .map(|ch| match ch {
            '.' => Ok(false),
            '#' => Ok(true),
            _ => Err("Unrecognized char in input")
        }).collect()
}

fn reduce(pt: (i32, i32)) -> (i32, i32) {
    let gcd = num_integer::gcd(pt.0, pt.1);
    if gcd > 0 {
        (pt.0 / gcd, pt.1 / gcd)
    } else {
        pt
    }
}

impl Map {
    fn load_from_string(s: &str) -> Result<Self, &'static str> {
        let data: Result<Vec<Vec<bool>>, &'static str> = s.lines().map(load_line).collect::<Result<Vec<_>, &'static str>>();
        let data = data?;
        let dx = data[0].len();
        let dy = data.len();
        Ok(Map{data, dx, dy})
    }

    fn at(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.dy {
            None
        } else {
            if x >= self.dx {
                None
            } else {
                Some(self.data[y][x])
            }
        }
    }

    fn asteroid_coordinates(&self) -> Vec<(i32,i32)>{
        let mut result = Vec::new();

        for (y, row) in self.data.iter().enumerate() {
            for (x, pt) in row.iter().enumerate() {
                if self.data[y][x] {
                    result.push((x as i32, y as i32));
                }
            }
        }

        result
    }

    fn count_position(&self, pos: (i32, i32)) -> i32 {
        let coords = self.asteroid_coordinates();
        let reachable_coords: BTreeSet<(i32, i32)> = coords.iter().map(|(x, y)| (x - pos.0, y - pos.1))
            .map(reduce)
            .filter(|p| *p != (0, 0))
            .collect();
        reachable_coords.len() as i32
    }

    fn count_best_position(&self) -> i32 {
        self.asteroid_coordinates().iter().map(|p| self.count_position(*p)).max().unwrap()
    }
}

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let map = Map::load_from_string(&fs::read_to_string(input_path).unwrap()).unwrap();

    println!("Part 1: {}", map.count_best_position());
}

#[test]
fn provided_case_1() {
    let input = 
".#..#
.....
#####
....#
...##";
    let map = Map::load_from_string(input).unwrap();
    assert_eq!(map.data, 
        vec!(
            vec!(false,true,false,false,true),
            vec!(false,false,false,false,false),
            vec!(true,true,true,true,true),
            vec!(false,false,false,false,true),
            vec!(false,false,false,true,true),
        ));

    assert_eq!(map.count_position((3, 4)), 8);

    assert_eq!(map.count_best_position(), 8);
}