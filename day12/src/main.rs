
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    fn from_string(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if !s.starts_with('<') || !s.ends_with('>') {
            return Err(format!("Could not parse '{}' as vector", s));
        }

        let s = &s[1..s.len() - 1];

        let mut x: Option<i32> = None;
        let mut y: Option<i32> = None;
        let mut z: Option<i32> = None;
        for component in s.split(',').map(|s| s.trim()) {
            let parts: Vec<&str> = component.split('=').collect();
            if parts.len() != 2 {
                return Err(format!("Could not parse '{}' as a component", component));
            }

            match parts[0].trim() {
                "x" => {
                    x = parts[1].parse().ok();
                }
                "y" => {
                    y = parts[1].parse().ok();
                }
                "z" => {
                    z = parts[1].parse().ok();
                }
                _ => {
                    return Err("Invalid component".to_string());
                }
            };
        }

        if x.is_none() || y.is_none() || z.is_none() { 
            return Err("Missing component: x,y,z required".to_string());
        }

        Ok(Vector3{ x: x.unwrap(), y: y.unwrap(), z: z.unwrap() })
    }

    fn new(x: i32, y: i32, z: i32) -> Self {
        Self{ x, y, z }
    }

    fn zero() -> Self {
        Vector3{x: 0, y: 0, z: 0}
    }

    fn add(v1: &Vector3, v2: &Vector3) -> Self {
        Self{ x: v1.x + v2.x, y: v1.y + v2.y, z: v1.z + v2.z }
    }

    fn sub(v1: &Vector3, v2: &Vector3) -> Self {
        Self{ x: v1.x - v2.x, y: v1.y - v2.y, z: v1.z - v2.z }
    }

    fn unit_scalar(v: i32) -> i32 {
        if v > 0 {
            1
        } else if v < 0 {
            -1
        } else {
            0
        }
    }

    fn unit(&self) -> Vector3 {
        Vector3{ x: Self::unit_scalar(self.x), y: Self::unit_scalar(self.y), z: Self::unit_scalar(self.z)}
    }

    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn get_z(&self) -> i32 {
        self.z
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
}

impl Moon {
    fn new(position: Vector3, velocity: Vector3) -> Self {
        Self{ position, velocity }
    }

    fn from_initial_position(p: &Vector3) -> Self {
        Moon{ position: p.clone(), velocity: Vector3::zero() }
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct System {
    moons: Vec<Moon>
}

impl System {
    fn from_file(path: &str) -> Result<Self, String> {
        let moons: Vec<Moon> = load_from_file(path)?.iter().map(Moon::from_initial_position).collect();
        Ok(Self{ moons })
    }

    fn from_string(s: &str) -> Result<Self, String> {
        let moons: Vec<Moon> = load_from_string(s)?.iter().map(Moon::from_initial_position).collect();
        Ok(Self{ moons })
    }

    fn apply_gravity(&mut self) {
        let moons_clone = self.moons.clone();
        for m1 in self.moons.iter_mut() {
            for m2 in moons_clone.iter() {
                let delta = Vector3::sub(&m2.position, &m1.position).unit();
                m1.velocity = Vector3::add(&m1.velocity, &delta);
            }
        }
    }

    fn apply_velocity(&mut self) {
        for m in self.moons.iter_mut() {
            m.position = Vector3::add(&m.position, &m.velocity);
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter()
            .map(|m| m.potential_energy() * m.kinetic_energy())
            .fold(0, |sum, e| sum + e)
    }
}

fn get_component_vector<F>(system: &System, component: F) -> Vec<i32> 
where F: Fn(&Vector3) -> i32 {
    let mut result = Vec::new();

    for moon in system.moons.iter() {
        result.push(component(&moon.position));
        result.push(component(&moon.velocity));
    }

    result
}

fn find_cycle(mut system: System) -> Option<u64> {
    

    let mut cycle_x: Option<u64> = None;
    let mut cycle_y: Option<u64> = None;
    let mut cycle_z: Option<u64> = None;
    let mut states_x: BTreeSet<Vec<i32>> = BTreeSet::new();
    let mut states_y: BTreeSet<Vec<i32>> = BTreeSet::new();
    let mut states_z: BTreeSet<Vec<i32>> = BTreeSet::new();

    for i in 0..1000000 {
        if cycle_x == Option::None {
            let component = get_component_vector(&system, Vector3::get_x);
            if states_x.contains(&component) {
                cycle_x = Some(i);
            } else {
                states_x.insert(component);
            }
        }

        if cycle_y == Option::None {
            let component = get_component_vector(&system, Vector3::get_y);
            if states_y.contains(&component) {
                cycle_y = Some(i);
            } else {
                states_y.insert(component);
            }
        }

        if cycle_z == Option::None {
            let component = get_component_vector(&system, Vector3::get_z);
            if states_z.contains(&component) {
                cycle_z = Some(i);
            } else {
                states_z.insert(component);
            }
        }

        if cycle_x != None && cycle_y != None && cycle_z != None {
            break;
        }

        system.step();
    }

    if cycle_x == None || cycle_y == None || cycle_z == None {
        None
    } else {
        let xy = num_integer::lcm(cycle_x.unwrap(), cycle_y.unwrap());
        Some(num_integer::lcm(xy, cycle_z.unwrap()))
    }
}

fn main() {
    let input_path: &String = &env::args().nth(1).unwrap();
    let mut system = System::from_file(input_path).unwrap();

    for _ in 0..1000000 {
        system.step();
    }

    println!("Part 1: {}", system.total_energy());

    let system = System::from_file(input_path).unwrap();
    let start = Instant::now();
    println!("Part 2: {:?}", find_cycle(system));
    let duration = Instant::now() - start;
    println!("Time: {}", duration.as_micros());
}

fn load_from_string(s: &str) -> Result<Vec<Vector3>, String> {
    s.trim().lines().map(Vector3::from_string).collect()
}

fn load_from_file(path: &str) -> Result<Vec<Vector3>, String> {
    let input_data = fs::read_to_string(path).map_err(|_| "Error reading file".to_string())?;
    load_from_string(&input_data)
}

#[test]
fn test_load_from_string() {
    assert_eq!(Vector3::from_string("<x=1,y=2,z=3>"), Ok(Vector3{x: 1, y: 2, z: 3}));

    assert_eq!(
        load_from_string("<x=1,y=2,z=3>\n<x=4,y=5,z=6>"),
        Ok(vec!(Vector3{x: 1, y: 2, z: 3}, Vector3{x: 4, y: 5, z: 6}))
)
}

#[test]
fn provided_test_1() {
    let mut system = System::from_string("
    <x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>
    ").unwrap();

    system.step();
    assert_eq!(system.moons[0], Moon::new(Vector3::new(2, -1, 1), Vector3::new(3, -1, -1)));
    assert_eq!(system.moons[1].velocity, Vector3::new(1, 3, 3));

    system.step();
    assert_eq!(system.moons[0].velocity, Vector3::new(3, -2, -2));
    assert_eq!(system.moons[1].velocity, Vector3::new(-2, 5, 6));

    system.step();
    assert_eq!(system.moons[0].velocity, Vector3::new(0, -3, 0));
    assert_eq!(system.moons[1].velocity, Vector3::new(-1, 2, 4));

    for _ in 3..10 {
        system.step();
    }

    assert_eq!(system.total_energy(), 179);
}

