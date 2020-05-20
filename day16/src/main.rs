use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let input_path: &str = &env::args().nth(1).unwrap();
    let input_data = fs::read_to_string(input_path).map_err(|_| "Couldn't read input file".to_string())?;
    let input = digits_of(input_data.trim());
    
    //part1(vec!(1, 2, 3, 4, 5))?;
    part1(input.clone())?;
    Ok(())
}

fn char_to_i8(ch: char) -> i8 {
    assert!(ch >= '0' && ch <= '9');
    (ch as i8) - ('0' as i8)
}

fn i8_to_char(i: i8) -> char {
    assert!(i >= 0 && i <= 9);
    (i + ('0' as i8)) as u8 as char
}

fn digits_of(s: &str) -> Vec<i8> {
    s.chars().map(char_to_i8).collect()
}

#[test]
fn test_digits_of() {
    assert_eq!(
        digits_of("12345"),
        vec!(1, 2, 3, 4, 5)
    );
}

fn make_row(i: usize, len: usize) -> Vec<i8> {
    let mut result: Vec<i8> = Vec::with_capacity(len);

    // adds more than necessary; we'll just take what we need at the end
    while result.len() < len + 1 {
        for x in [0, 1, 0, -1].iter() {
            for _ in 0..(i + 1) {
                result.push(*x);
            }
        }
    }

    result[1..(len + 1)].iter().cloned().collect()
}

#[test]
fn test_make_row() {
    assert_eq!(
        make_row(0, 10),
        vec!(1, 0, -1, 0, 1, 0, -1, 0, 1, 0)
    );

    assert_eq!(
        make_row(1, 10),
        vec!(0, 1, 1, 0, 0, -1, -1, 0, 0, 1)
    );

    assert_eq!(
        make_row(2, 10),
        vec!(0, 0, 1, 1, 1, 0, 0, 0, -1, -1)
    );
}

fn make_pattern(len: usize) -> Vec<Vec<i8>> {
    (0..len).map(|i| make_row(i, len)).collect()
}

fn last_digit_of(i: i32) -> i8 {
    (i.abs() % 10) as i8
}

fn apply_fft(input: &[i8], pattern: &Vec<Vec<i8>>) -> Vec<i8> {
    let mut result: Vec<i8> = Vec::with_capacity(input.len());
    let multiply = |(x, y): (&i8, &i8)| {
        assert!(*x <= 9);
        assert!(*y <= 9);
        (*x as i32) * (*y as i32)
    };
    for i in 0..input.len() {
        let digit = input.iter().zip(pattern[i].iter()).map(multiply).sum();
        result.push(last_digit_of(digit));
    }

    result
}

#[test]
fn test_apply_fft_1() {
    assert_eq!(
        apply_fft(&digits_of("12345678"), &make_pattern(8)),
        digits_of("48226158")
    );
}

#[test]
fn test_apply_fft_2() {
    let mut message = digits_of("80871224585914546619083218645595");
    let pattern = make_pattern(message.len());

    for _ in 0..100 {
        message = apply_fft(&message, &pattern)
    }

    assert_eq!(
        &message[0..8],
        &digits_of("24176176")[..]
    )
}

fn part1(mut message: Vec<i8>) -> Result<(), String> {
    let pattern = make_pattern(message.len());
    
    for _ in 0..100 {
        message = apply_fft(&message, &pattern)
    }

    println!("Part 1: {:?}", &message[0..8]);
    Ok(())
}