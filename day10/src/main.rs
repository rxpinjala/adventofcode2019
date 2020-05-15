
use std::collections::BTreeSet;
use std::env;
use std::fs;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct CartesianPoint {
    x: i32,
    y: i32,
}

impl CartesianPoint {
    fn new(x: i32, y: i32) -> Self {
        CartesianPoint{x, y}
    }

    fn origin() -> Self {
        CartesianPoint{x: 0, y: 0}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct PolarPoint {
    rho: f32,
    theta: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Asteroid {
    abs: CartesianPoint,
    rel: CartesianPoint,
    polar: PolarPoint,
}

impl Asteroid {
    fn relative_to(pt: &CartesianPoint, relative_to: &CartesianPoint) -> Self {
        let rel_pt = CartesianPoint{ x: pt.x - relative_to.x, y: pt.y - relative_to.y };
        let rho_squared: i32 = (rel_pt.x * rel_pt.x) + (rel_pt.y * rel_pt.y);
        let rho = (rho_squared as f32).sqrt();

        let rel_pt_reduced = nearest_coordinate(rel_pt);

        let theta = ((rel_pt_reduced.y) as f32).atan2(rel_pt_reduced.x as f32); // range -pi..pi
        let theta = theta + std::f32::consts::FRAC_PI_2; // rotate 90 degrees, range -pi/2..3pi/2
        let theta = if theta < 0.0 { theta + std::f32::consts::PI * 2.0 } else { theta }; // shift range to 0..2pi
        let polar = PolarPoint{ rho, theta };
        Asteroid{ abs: *pt, rel: rel_pt, polar }
    }
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

fn unit(f: i32) -> i32 {
    if f < 0 {
        -1
    } else if f > 0 {
        1
    } else {
        0
    }
}

fn nearest_coordinate(pt: CartesianPoint) -> CartesianPoint {
    if pt.x == 0 || pt.y == 0 {
        return CartesianPoint{x: unit(pt.x), y: unit(pt.y)};
    }

    let gcd = num_integer::gcd(pt.x, pt.y);
    CartesianPoint{x: pt.x / gcd, y: pt.y / gcd}
}

struct Map {
    data: Vec<CartesianPoint>
}

impl Map {
    fn load_from_string(s: &str) -> Result<Self, &'static str> {
        let raw_data: Vec<Vec<bool>> = s.lines().map(load_line).collect::<Result<Vec<_>, &'static str>>()?;

        let mut data = Vec::new();

        for (y, row) in raw_data.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value {
                    data.push(CartesianPoint{ x: x as i32, y: y as i32 });
                }
            }
        }

        Ok(Map{data})
    }

    fn count_position(&self, pos: &CartesianPoint) -> i32 {
        self.data.iter()
            .filter(|pt| *pt != pos)
            .map(|pt| Asteroid::relative_to(pt, pos))
            .map(|a| nearest_coordinate(a.rel))
            .collect::<BTreeSet<CartesianPoint>>()
            .len() as i32
    }

    fn count_best_position(&self) -> (CartesianPoint, i32) {
        self.data.iter().map(|p| (*p, self.count_position(p))).max_by_key(|(_p, c)| *c).unwrap()
    }

    fn laser_asteroids(&self, origin: CartesianPoint) -> Vec<CartesianPoint> {
        let mut asteroids: Vec<Asteroid> = self.data.iter()
            .filter(|pt| **pt != origin)
            .map(|pt| Asteroid::relative_to(pt, &origin))
            .collect();
        asteroids.sort_by(|a1, a2| a1.polar.theta.partial_cmp(&a2.polar.theta).unwrap());

        let mut groups = asteroids.iter()
            .group_by(|a| a.polar.theta)
            .into_iter()
            .map(|(_, group)| group.cloned().collect())
            .collect::<Vec<Vec<Asteroid>>>();
        
        //dbg!(&groups);
        
        let mut result: Vec<CartesianPoint> = Vec::new();
        while groups.len() > 0 {
            for group in &mut groups {
                let nearest_index = group.iter().enumerate()
                    .min_by_key(|a| a.1.polar.rho as i32).unwrap().0;

                result.push(group[nearest_index].abs);
                group.remove(nearest_index);
            }

            groups.retain(|group| group.len() > 0);
        }

        result
    }
}

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let map = Map::load_from_string(&fs::read_to_string(input_path).unwrap()).unwrap();

    let best = map.count_best_position();
    println!("Part 1: {:?}", &best);

    let order = map.laser_asteroids(best.0);
    println!("Part 2: {:?}", order[199]);
}

#[test]
fn test_asteroid_math() {
    let a = Asteroid::relative_to(&CartesianPoint::new(0, -1), &CartesianPoint::origin());
    dbg!(&a);
    assert_eq!(a.polar.rho, 1.0);
    assert_eq!(a.polar.theta, 0.0);

    let a = Asteroid::relative_to(&CartesianPoint::new(1, 0), &CartesianPoint::origin());
    dbg!(&a);
    assert_eq!(a.polar.rho, 1.0);
    assert_eq!(a.polar.theta, std::f32::consts::PI / 2.0);

    let a = Asteroid::relative_to(&CartesianPoint::new(0, 1), &CartesianPoint::origin());
    dbg!(&a);
    assert_eq!(a.polar.rho, 1.0);
    assert_eq!(a.polar.theta, std::f32::consts::PI);

    let a = Asteroid::relative_to(&CartesianPoint::new(-1, 0), &CartesianPoint::origin());
    dbg!(&a);
    assert_eq!(a.polar.rho, 1.0);
    assert_eq!(a.polar.theta, std::f32::consts::PI * 3.0 / 2.0);
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

    assert_eq!(map.count_position(&CartesianPoint{x: 3, y: 4}), 8);

    assert_eq!(map.count_best_position().1, 8);
}

#[test]
fn test_laser_order() {
    let input = 
"###
###
###";
    let map = Map::load_from_string(&input).unwrap();
    
    let order = map.laser_asteroids(CartesianPoint::new(1, 1));
    assert_eq!(order, vec!(
        CartesianPoint::new(1, 0),
        CartesianPoint::new(2, 0),
        CartesianPoint::new(2, 1),
        CartesianPoint::new(2, 2),
        CartesianPoint::new(1, 2),
        CartesianPoint::new(0, 2),
        CartesianPoint::new(0, 1),
        CartesianPoint::new(0, 0),
    ));

    let order = map.laser_asteroids(CartesianPoint::new(0, 0));
    assert_eq!(order, vec!(
        CartesianPoint::new(1, 0),
        CartesianPoint::new(2, 1),
        CartesianPoint::new(1, 1),
        CartesianPoint::new(1, 2),
        CartesianPoint::new(0, 1),
        CartesianPoint::new(2, 0),
        CartesianPoint::new(2, 2),
        CartesianPoint::new(0, 2),
    ));
}

#[test]
fn provided_case_part2() {
    let input =
".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
    let map = Map::load_from_string(&input).unwrap();

    let order = map.laser_asteroids(CartesianPoint::new(8, 3));
    dbg!(&order);
    assert_eq!(order[0], CartesianPoint::new(8, 1));
    assert_eq!(order[1], CartesianPoint::new(9, 0));
    assert_eq!(order[2], CartesianPoint::new(9, 1));
    assert_eq!(order[3], CartesianPoint::new(10, 0));
    assert_eq!(order[4], CartesianPoint::new(9, 2));
    assert_eq!(order[5], CartesianPoint::new(11, 1));
    assert_eq!(order[6], CartesianPoint::new(12, 1));
    assert_eq!(order[7], CartesianPoint::new(11, 2));
    assert_eq!(order[8], CartesianPoint::new(15, 1));
    assert_eq!(order[9], CartesianPoint::new(12, 2));
    assert_eq!(order[10], CartesianPoint::new(13, 2));
    assert_eq!(order[11], CartesianPoint::new(14, 2));
    assert_eq!(order[12], CartesianPoint::new(15, 2));
}

#[test]
fn provided_case_large() {
    let input = 
".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    let map = Map::load_from_string(&input).unwrap();
    let order = map.laser_asteroids(CartesianPoint::new(11, 13));

    dbg!(&order);
    assert_eq!(order.len(), 299);
    assert_eq!(order[1-1], CartesianPoint::new(11, 12));
    assert_eq!(order[2-1], CartesianPoint::new(12, 1));
    assert_eq!(order[3-1], CartesianPoint::new(12, 2));
    assert_eq!(order[10-1], CartesianPoint::new(12, 8));
    assert_eq!(order[20-1], CartesianPoint::new(16, 0));
    assert_eq!(order[50-1], CartesianPoint::new(16, 9));
    assert_eq!(order[100-1], CartesianPoint::new(10, 16));
    assert_eq!(order[199-1], CartesianPoint::new(9, 6));
    assert_eq!(order[200-1], CartesianPoint::new(8, 2));
    assert_eq!(order[201-1], CartesianPoint::new(10, 9));
    assert_eq!(order[299-1], CartesianPoint::new(11, 1));
}