const INPUT: &str = include_str!("../../input/day_1.txt");

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

pub fn part_1(input: &str) -> usize {
    input.lines().map(parse_first_last_digit).sum::<usize>()
}
pub fn part_2(input: &str) -> usize {
    let pattern_replace = &[
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    input
        .lines()
        .map(|l| {
            pattern_replace
                .iter()
                .fold(l.to_string(), |s, (pattern, replace)| {
                    s.replace(pattern, replace)
                })
        })
        .map(parse_first_last_digit)
        .sum::<usize>()
}

fn parse_first_last_digit<S: AsRef<str>>(l: S) -> usize {
    let digits = l
        .as_ref()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<Vec<_>>();
    let mut s = String::new();
    s.push(*digits.first().unwrap());
    s.push(*digits.last().unwrap());
    s.parse::<usize>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_INPUT_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    const TEST_INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT_1), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT_2), 281);
    }
}
