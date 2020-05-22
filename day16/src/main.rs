use std::env;
use std::fs;

type Num = i64;

fn main() {
    let input_path: &str = &env::args().nth(1).unwrap();
    let input_data = fs::read_to_string(input_path).unwrap();
    let input = digits_of(input_data.trim());

    //part1(vec!(1, 2, 3, 4, 5))?;
    part1(&input);
    part2(&input);
}

fn char_to_digit(ch: char) -> Num {
    assert!(ch >= '0' && ch <= '9');
    (ch as Num) - ('0' as Num)
}

fn digit_to_char(i: Num) -> char {
    assert!(i >= 0 && i <= 9);
    (i + ('0' as Num)) as u8 as char
}

fn digits_of(s: &str) -> Vec<Num> {
    s.chars().map(char_to_digit).collect()
}

#[test]
fn test_digits_of() {
    assert_eq!(digits_of("12345"), vec!(1, 2, 3, 4, 5));
}

fn get_message_offset(input: &[Num]) -> usize {
    input
        .iter()
        .copied()
        .map(digit_to_char)
        .fold(String::new(), |acc, ch| format!("{}{}", acc, ch))
        .parse::<usize>()
        .unwrap()
}

#[test]
fn test_get_message_offset() {
    assert_eq!(
        get_message_offset(&[1, 2, 3, 4, 5]),
        12345
    );
}

fn part1(message: &[Num]) {
    let mut message = message.to_vec();

    for _ in 0..100 {
        message = apply_fft_faster(&message)
    }

    println!("Part 1: {:?}", &message[0..8]);
}

fn part2(input: &[Num]) {
    let count = 10_000;
    let mut message: Vec<Num> = Vec::with_capacity(input.len() * count);
    for _ in 0..count {
        message.extend_from_slice(input);
    }

    for i in 0..100 {
        println!("Iteration {}", i);
        message = apply_fft_faster(&message);
    }

    let message_offset = get_message_offset(&input[0..7]);

    println!("Part 2: {:?}", &message[message_offset .. (message_offset + 8)])
}

#[derive(Debug)]
struct SumTreeNode<'a> {
    data: &'a [Num],
    sum: Num,
    left_child: Option<Box<SumTreeNode<'a>>>,
    right_child: Option<Box<SumTreeNode<'a>>>,
}

impl<'a> SumTreeNode<'a> {
    fn new(data: &'a [Num]) -> Self {
        if data.len() < 2 {
            unreachable!();
        }

        let mid = data.len() / 2;
        let left_range = &data[..mid];
        let right_range = &data[mid..];

        let left_child = if left_range.len() > 4 {
            Some(Box::new(Self::new(
                left_range,
            )))
        } else {
            None
        };
        let right_child = if right_range.len() > 4 {
            Some(Box::new(Self::new(
                right_range,
            )))
        } else {
            None
        };

        assert!(left_child.is_none() == right_child.is_none());

        let left_sum = match left_child.as_ref() {
            Some(child) => child.sum,
            None => left_range.iter().sum(),
        };
        let right_sum = match right_child.as_ref() {
            Some(child) => child.sum,
            None => right_range.iter().sum(),
        };
        let sum = left_sum + right_sum;

        Self {
            data,
            sum,
            left_child,
            right_child,
        }
    }

    fn has_children(&self) -> bool {
        self.left_child.is_some()
    }

    fn sum_of_range(&self, start: usize, end: usize) -> i64 {
        let start = std::cmp::min(start, self.data.len());
        let end = std::cmp::min(end, self.data.len());

        if start == end {
            return 0;
        }

        if start == 0 && end == self.data.len() {
            return self.sum;
        }

        if self.has_children() {
            let mid = self.left_child.as_ref().unwrap().data.len();
            if end <= mid {
                self.left_child.as_ref().unwrap().sum_of_range(start, end)
            } else if start >= mid {
                self.right_child.as_ref().unwrap().sum_of_range(start - mid, end - mid)
            } else {
                let left_sum = self.left_child.as_ref().unwrap().sum_of_range(start, mid);
                let right_sum = self.right_child.as_ref().unwrap().sum_of_range(0, end - mid);
                left_sum + right_sum
            }
        } else {
            self.data[start..end].iter().copied().sum()
        }
    }
}

#[test]
fn test_sum_of_range() {
    let node = SumTreeNode::new(
        &[0,1,2,3,4,5,6,7,8,9]
    );

    assert_eq!(node.sum_of_range(0, 8), 28);
    assert_eq!(node.sum_of_range(0, 4), 6);
    assert_eq!(node.sum_of_range(2, 6), 14);
}

fn apply_fft_faster(input: &[Num]) -> Vec<Num> {
    let mut result: Vec<Num> = Vec::new();
    let sum_tree = SumTreeNode::new(input);

    //dbg!(&sum_tree);

    for row in 0..input.len() {
        if row % 10000 == 0 {
            println!("  Row: {}", row);
        }
        let pattern_len = row + 1;
        let mut sum: Num = 0;

        let mut start = pattern_len - 1;
        while start < input.len() {
            let add_start = start;
            let add_end = add_start + pattern_len;
            let sub_start = add_end + pattern_len;
            let sub_end = sub_start + pattern_len;
            sum = sum + 
                sum_tree.sum_of_range(add_start, add_end) -
                sum_tree.sum_of_range(sub_start, sub_end);

            start += pattern_len * 4;
        }

        result.push((sum.abs() % 10) as Num);
    }

    result
}

#[test]
fn test_apply_fft_faster() {
    assert_eq!(apply_fft_faster(&digits_of("12345678")), digits_of("48226158"));

    let mut message = digits_of("80871224585914546619083218645595");

    for _ in 0..100 {
        message = apply_fft_faster(&message)
    }

    assert_eq!(&message[0..8], &digits_of("24176176")[..])
}

