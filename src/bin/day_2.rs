use crate::Color::{Blue, Green, Red};
use anyhow::anyhow;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::max;
use std::str::FromStr;

const INPUT: &str = include_str!("../../input/day_2.txt");

#[derive(Debug)]
struct Game {
    idx: usize,
    sets: Vec<Vec<Draw>>,
}

#[derive(Default)]
struct Power {
    red: usize,
    green: usize,
    blue: usize,
}

impl Power {
    pub fn update_max(&mut self, draw: &Draw) {
        match draw.color {
            Blue => self.blue = max(self.blue, draw.count),
            Red => self.red = max(self.red, draw.count),
            Green => self.green = max(self.green, draw.count),
        }
    }

    pub fn power(&self) -> usize {
        self.green * self.blue * self.red
    }
}

impl Game {
    pub fn is_valid(&self, specs: &[Draw], filter: fn(&Draw, &Draw) -> bool) -> bool {
        specs.iter().all(|spec| {
            self.sets
                .iter()
                .all(|set| set.iter().all(|draw| filter(draw, spec)))
        })
    }

    pub fn power(&self) -> usize {
        let mut max = Power::default();
        self.sets
            .iter()
            .for_each(|set| set.iter().for_each(|draw| max.update_max(draw)));
        max.power()
    }
}

static GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Game (\d+):").unwrap());

static SET_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*(\d+) (blue|red|green)\s*$").unwrap());

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx: usize = GAME_REGEX
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let sets: Vec<Vec<Draw>> = GAME_REGEX
            .replace(s, "")
            .split(';')
            .map(|sets| {
                sets.split(',')
                    .map(|set| {
                        let caps = SET_REGEX.captures(set).unwrap();
                        Draw {
                            count: caps.get(1).unwrap().as_str().parse().unwrap(),
                            color: Color::from_str(caps.get(2).unwrap().as_str()).unwrap(),
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Game { idx, sets })
    }
}

#[derive(Debug)]
struct Draw {
    count: usize,
    color: Color,
}

impl Draw {
    pub fn new(count: usize, color: Color) -> Self {
        Self { count, color }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Color {
    Blue,
    Red,
    Green,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Red),
            "blue" => Ok(Blue),
            "green" => Ok(Green),
            _ => Err(anyhow!("unknown color")),
        }
    }
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}

pub fn part_1(input: &str) -> usize {
    let games: Vec<Game> = input.lines().map(|l| Game::from_str(l).unwrap()).collect();

    let spec = vec![
        Draw::new(12, Red),
        Draw::new(13, Green),
        Draw::new(14, Blue),
    ];

    games
        .iter()
        .filter(|g| {
            g.is_valid(&spec, |set, spec| {
                set.color != spec.color || set.count <= spec.count
            })
        })
        .map(|g| g.idx)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let games: Vec<Game> = input.lines().map(|l| Game::from_str(l).unwrap()).collect();
    games.iter().map(Game::power).sum()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 2286);
    }
}
