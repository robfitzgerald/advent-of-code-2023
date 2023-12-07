use std::{collections::HashMap, str::FromStr};

use crate::day::Day;

#[derive(PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    const ALL: [Color; 3] = [Color::Red, Color::Green, Color::Blue];
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(format!("unknown color {}", s)),
        }
    }
}

pub fn parse_draw(draw: &str) -> Result<(Color, u64), String> {
    let tokens: Vec<&str> = draw.trim().split(" ").collect();
    let count: u64 = tokens[0].parse::<u64>().map_err(|e| e.to_string())?;
    let color = Color::from_str(tokens[1])?;
    Ok((color, count))
}

pub fn min(a: u64, b: u64) -> u64 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn max(a: u64, b: u64) -> u64 {
    if a < b {
        b
    } else {
        a
    }
}

#[derive(Default)]
pub struct GameSet {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

pub fn possible_game(rubric: &GameSet, test: &GameSet) -> bool {
    test.red <= rubric.red && test.green <= rubric.green && test.blue <= rubric.blue
}

impl GameSet {
    // pub fn max_reducer(&self, that: &GameSet) -> GameSet {
    //     GameSet {
    //         red: max(self.red, that.red),
    //         green: max(self.green, that.green),
    //         blue: max(self.blue, that.blue),
    //     }
    // }

    const PROBLEM_1_CONSTRAINT: GameSet = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };
}

pub fn max_reducer(this: &GameSet, that: &GameSet) -> GameSet {
    GameSet {
        red: max(this.red, that.red),
        green: max(this.green, that.green),
        blue: max(this.blue, that.blue),
    }
}

pub fn min_reducer(this: &GameSet, that: &GameSet) -> GameSet {
    GameSet {
        red: max(this.red, that.red),
        green: max(this.green, that.green),
        blue: max(this.blue, that.blue),
    }
}

pub fn p1_collect(game_set: &GameSet, game_id: u64) -> Option<u64> {
    let possible_game = possible_game(&GameSet::PROBLEM_1_CONSTRAINT, &game_set);
    let result = if possible_game { Some(game_id) } else { None };
    result
}

pub fn p2_collect(game_set: &GameSet, _game_id: u64) -> Option<u64> {
    let power_of_set = game_set.red * game_set.green * game_set.blue;
    Some(power_of_set)
}

impl FromStr for GameSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .trim()
            .split(",")
            .map(parse_draw)
            .collect::<Result<HashMap<Color, u64>, String>>()?;
        let block_count = GameSet {
            red: *tokens.get(&Color::Red).unwrap_or(&0),
            green: *tokens.get(&Color::Green).unwrap_or(&0),
            blue: *tokens.get(&Color::Blue).unwrap_or(&0),
        };
        Ok(block_count)
    }
}

pub fn solve(
    document: String,
    reducer: &dyn Fn(&GameSet, &GameSet) -> GameSet,
    collector: &dyn Fn(&GameSet, u64) -> Option<u64>,
) -> Result<Vec<Option<u64>>, String> {
    document
        .lines()
        .map(|game| {
            let initial_split: Vec<&str> = game[5..].split(":").collect();
            let game_id = initial_split[0].parse::<u64>().map_err(|e| e.to_string())?;
            let game_sets = initial_split[1]
                .trim()
                .split(";")
                .map(GameSet::from_str)
                .collect::<Result<Vec<_>, String>>()?;
            let reduced = game_sets
                .iter()
                .fold(GameSet::default(), |a, b| reducer(&a, b));
            let result = collector(&reduced, game_id);
            // let possible_game = possible_game(&constraint, &reduced);
            // let result = if possible_game { Some(game_id) } else { None };
            Ok(result)
        })
        .collect::<Result<Vec<_>, String>>()
}

pub fn run(puzzle_filename: &String, day: &Day) -> Result<String, String> {
    let document = std::fs::read_to_string(puzzle_filename).map_err(|e| e.to_string())?;
    let games_result = match day {
        Day::Day2Part1 => solve(document, &max_reducer, &p1_collect)?,
        Day::Day2Part2 => solve(document, &min_reducer, &p2_collect)?,
        _ => Err(format!("unsupported day input {:?}", day))?,
    };

    let sum: u64 = games_result.iter().flatten().sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
}
