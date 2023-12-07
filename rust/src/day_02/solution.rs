use std::{collections::HashMap, str::FromStr};

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
    pub fn max_reducer(&self, that: &GameSet) -> GameSet {
        GameSet {
            red: max(self.red, that.red),
            green: max(self.green, that.green),
            blue: max(self.blue, that.blue),
        }
    }
}

pub fn max_reducer(this: &GameSet, that: &GameSet) -> GameSet {
    GameSet {
        red: max(this.red, that.red),
        green: max(this.green, that.green),
        blue: max(this.blue, that.blue),
    }
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

pub fn run_day_2_pt_1(puzzle_filename: &String) -> Result<String, String> {
    let document = std::fs::read_to_string(puzzle_filename).map_err(|e| e.to_string())?;
    let constraint = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games_result = document
        .lines()
        .map(|game| {
            let initial_split: Vec<&str> = game[5..].split(":").collect();
            let game_id = initial_split[0].parse::<u64>().map_err(|e| e.to_string())?;
            let game_sets = initial_split[1]
                .trim()
                .split(";")
                .map(GameSet::from_str)
                .collect::<Result<Vec<_>, String>>()?;
            let max_game_set = game_sets
                .iter()
                .fold(GameSet::default(), |a, b| a.max_reducer(b));
            let possible_game = possible_game(&constraint, &max_game_set);
            let result = if possible_game { Some(game_id) } else { None };
            Ok(result)
        })
        .collect::<Result<Vec<_>, String>>()?;
    let sum: u64 = games_result.iter().flatten().sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
}
