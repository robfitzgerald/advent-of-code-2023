use std::str::FromStr;

use crate::{day_01, day_02, day_03};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Day {
    Day1Part1,
    Day1Part2,
    Day2Part1,
    Day2Part2,
    Day3Part1,
    Day3Part2,
}

impl Day {
    pub fn run(&self, puzzle_filename: &String) -> Result<String, String> {
        match self {
            Day::Day1Part1 => day_01::solution::run_day_01(puzzle_filename, false),
            Day::Day1Part2 => day_01::solution::run_day_01(puzzle_filename, true),
            Day::Day2Part1 => day_02::solution::run(puzzle_filename, self),
            Day::Day2Part2 => day_02::solution::run(puzzle_filename, self),
            Day::Day3Part1 => day_03::solution::problem_01(puzzle_filename),
            Day::Day3Part2 => day_03::solution::problem_02(puzzle_filename),
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
            "day_2_part_2" => Ok(Day::Day2Part2),
            "day_3_part_1" => Ok(Day::Day3Part1),
            "day_3_part_2" => Ok(Day::Day3Part2),
            _ => Err(format!(
                "unknown day {} should have format 'day_$day_part_$part'",
                s
            )),
        }
    }
}
