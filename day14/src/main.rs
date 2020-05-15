use std::collections::{BTreeMap};
use std::env;
use std::fs;

#[derive(Clone, Eq, PartialEq)]
struct ChemicalAmount {
    name: String,
    quantity: u32,
}

impl ChemicalAmount {
    fn parse(s: &str) -> Result<ChemicalAmount, String> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.len() == 2 {
            let quantity: u32 = parts[0].parse()
                .map_err(|_| format!("Couldn't parse quantity: '{}'", parts[0]))?;
            
            Ok(Self {
                name: parts[1].trim().to_string(),
                quantity
            })
        } else {
            Err(format!("Failed to parse chemical amount: '{}'", s))
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}", self.quantity, self.name)
    }
}

impl std::fmt::Debug for ChemicalAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn test_chemicalamount_parse() {
    assert_eq!(ChemicalAmount::parse("10 abc"), Ok(ChemicalAmount { name: "abc".to_string(), quantity: 10 }));
}

#[derive(Clone, Eq, PartialEq)]
struct Reaction {
    inputs: Vec<ChemicalAmount>,
    output: ChemicalAmount,
}

impl Reaction {
    fn parse_line(line: &str) -> Result<Reaction, String> {
        let i = line.find("=>").ok_or(format!("Invalid reaction: '{}'", line))?;
        let (left, right) = line.split_at(i);
        let right = &right[2..];

        let output = ChemicalAmount::parse(right)?;
        let inputs = left.split(',').map(ChemicalAmount::parse).collect::<Result<Vec<ChemicalAmount>, String>>()?;

        Ok(Self { inputs, output })
    }

    fn to_string(&self) -> String {
        let input_str = self.inputs.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");
        format!("<{} => {}>", input_str, self.output.to_string())
    }
}

impl std::fmt::Debug for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn test_reaction_parse_line() {
    assert_eq!(
        Reaction::parse_line("10 A => 1 B"),
        Ok(Reaction {
            inputs: vec!(
                ChemicalAmount {
                    name: "A".to_string(),
                    quantity: 10 }
                ),
            output: ChemicalAmount {
                name: "B".to_string(),
                quantity: 1
            }
        })
    );
}

fn main() -> Result<(), String> {
    let input_path: &str = &env::args().nth(1).unwrap();
    let reactions = fs::read_to_string(input_path)
        .map_err(|_| "Error reading input file".to_string())?
        .lines()
        .map(Reaction::parse_line)
        .collect::<Result<Vec<Reaction>, String>>()?;
    
    part1(&reactions);
    part2(&reactions);
    Ok(())
}

fn part1(reactions: &[Reaction]) {
    println!("Part 1: {}", compute_ore_cost(reactions, 1));
}

fn get_multiple(num: u64, denom: u64) -> u64 {
    let q = num / denom;
    let r = num % denom;
    if r == 0 {
        q
    } else {
        q + 1
    }
}

struct ReactionCount<'a> {
    reaction: &'a Reaction,
    count: u64,
}

fn find_missing_requirements<'a>(reaction_counts: &[ReactionCount<'a>]) -> BTreeMap<&'a str, i64> {
    let mut result: BTreeMap<&str, i64> = BTreeMap::new();

    for rc in reaction_counts.iter() {
        for input in rc.reaction.inputs.iter() {
            let consumed_quantity = (input.quantity as u64 * rc.count) as i64;
            let quantity = result.entry(&input.name).or_insert(0);
            *quantity = *quantity - consumed_quantity;
        }

        let output = &rc.reaction.output;
        let produced_quantity = (output.quantity as u64 * rc.count) as i64;
        let quantity = result.entry(&output.name).or_insert(0);
        *quantity = *quantity + produced_quantity;
    }

    result.into_iter().filter(|(_, v)| *v < 0).map(|(k, v)| (k, -v)).collect()
}

fn compute_ore_cost(reactions: &[Reaction], needed_fuel: u64) -> u64 {
    let mut reaction_counts: Vec<ReactionCount> = reactions.iter().map(|r| ReactionCount { reaction: r, count: 0 }).collect();

    for rc in reaction_counts.iter_mut() {
        if rc.reaction.output.name == "FUEL" {
            rc.count = needed_fuel;
        }
    }

    let mut missing_requirements = find_missing_requirements(&reaction_counts);

    let mut max_iterations = 10000;
    while missing_requirements.len() > 1 || !missing_requirements.contains_key("ORE") {
        let (missing_chemical, needed_count) = missing_requirements.iter().filter(|(chemical, _)| **chemical != "ORE").nth(0).unwrap();

        for rc in reaction_counts.iter_mut() {
            if rc.reaction.output.name == *missing_chemical {
                let multiple = get_multiple(*needed_count as u64, rc.reaction.output.quantity as u64);
                rc.count = rc.count + multiple;
            }
        }

        let new_missing_requirements = find_missing_requirements(&reaction_counts);
        if new_missing_requirements == missing_requirements {
            dbg!(missing_chemical);
            dbg!(needed_count);
            panic!("Not making progress");
        }
        missing_requirements = new_missing_requirements;

        max_iterations = max_iterations - 1;
        if max_iterations == 0 {
            panic!("Too many iterations");
        }
    }

    missing_requirements["ORE"] as u64
}

#[test]
fn test_case_0() {
    let reactions = vec!(
        Reaction::parse_line("2 ORE => 1 A").unwrap(),
        Reaction::parse_line("2 A => 1 FUEL").unwrap(),
    );

    assert_eq!(4, compute_ore_cost(&reactions[..], 1));
}

#[test]
fn test_case_1() {
    let reactions = vec!(
        Reaction::parse_line("10 ORE => 10 A").unwrap(),
        Reaction::parse_line("1 ORE => 1 B").unwrap(),
        Reaction::parse_line("7 A, 1 B => 1 C").unwrap(),
        Reaction::parse_line("7 A, 1 C => 1 D").unwrap(),
        Reaction::parse_line("7 A, 1 D => 1 E").unwrap(),
        Reaction::parse_line("7 A, 1 E => 1 FUEL").unwrap()
    );

    assert_eq!(31, compute_ore_cost(&reactions[..], 1));
}

#[test]
fn test_case_2() {
    let reactions = vec!(
        Reaction::parse_line("9 ORE => 2 A").unwrap(),
        Reaction::parse_line("8 ORE => 3 B").unwrap(),
        Reaction::parse_line("7 ORE => 5 C").unwrap(),
        Reaction::parse_line("3 A, 4 B => 1 AB").unwrap(),
        Reaction::parse_line("5 B, 7 C => 1 BC").unwrap(),
        Reaction::parse_line("4 C, 1 A => 1 CA").unwrap(),
        Reaction::parse_line("2 AB, 3 BC, 4 CA => 1 FUEL").unwrap(),
    );

    assert_eq!(165, compute_ore_cost(&reactions[..], 1));
}

fn part2(reactions: &[Reaction]) {
    let available_ore: u64 = 1000000000000;

    let mut lower_bound = 1;
    let mut upper_bound = 1000000000;

    assert!(compute_ore_cost(reactions, upper_bound) > available_ore);

    while (upper_bound - lower_bound) > 1 {
        let mid = (upper_bound + lower_bound) / 2;
        let ore_cost = compute_ore_cost(reactions, mid);
        if ore_cost > available_ore {
            upper_bound = mid;
        } else {
            lower_bound = mid;
        }
    }

    println!("Part 2: {}", lower_bound);
}