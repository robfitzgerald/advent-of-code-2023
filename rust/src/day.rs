use std::str::FromStr;

use crate::{day_01, day_02};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Day {
    Day1Part1,
    Day1Part2,
    Day2Part1,
}

impl Day {
    pub fn run(&self, puzzle_filename: &String) -> Result<String, String> {
        match self {
            Day::Day1Part1 => day_01::solution::run_day_01(puzzle_filename, false),
            Day::Day1Part2 => day_01::solution::run_day_01(puzzle_filename, true),
            Day::Day2Part1 => day_02::solution::run_day_2_pt_1(puzzle_filename),
        }
    }
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day_1_part_1" => Ok(Day::Day1Part1),
            "day_1_part_2" => Ok(Day::Day1Part2),
            "day_2_part_1" => Ok(Day::Day2Part1),
            _ => Err(format!(
                "unknown day {} should have format 'day_$day_part_$part'",
                s
            )),
        }
    }
}
