use std::collections::HashSet;

pub fn count_wins(row: &str) -> Result<u64, String> {
    let colon_split: Vec<&str> = row.split(":").collect();
    let pipe_split: Vec<&str> = colon_split[1].split("|").collect();
    let win_cards = pipe_split[0]
        .trim()
        .split(" ")
        .filter(|n_str| !n_str.is_empty())
        .map(|n_str| n_str.trim().parse::<u64>().map_err(|e| e.to_string()))
        .collect::<Result<HashSet<_>, String>>()?;
    let cards = pipe_split[1]
        .trim()
        .split(" ")
        .filter(|n_str| !n_str.is_empty())
        .map(|n_str| n_str.trim().parse::<u64>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, String>>()?;
    let n_wins = cards.iter().fold(0, |acc, card| {
        if win_cards.contains(card) {
            acc + 1
        } else {
            acc
        }
    });
    Ok(n_wins as u64)
}

pub fn score_wins(win_count: u64) -> u64 {
    if win_count == 0 {
        0u64
    } else {
        2u64.pow((win_count - 1) as u32)
    }
}

pub fn problem_02(puzzle_filename: &String) -> Result<String, String> {
    let document = std::fs::read_to_string(puzzle_filename).map_err(|e| e.to_string())?;
    let matches = document
        .lines()
        .map(count_wins)
        .collect::<Result<Vec<u64>, String>>()?;

    // update copies based on game rules
    let mut copies = vec![1; matches.len()];
    for i in 0..matches.len() {
        let match_count = matches[i] as usize;
        let copy_count = copies[i]; // as usize;
                                    // for _ in 0..copy_count {
        for k in 0..match_count {
            let k_idx = i + k + 1;
            copies[k_idx] += copy_count;
        }
        // }
    }

    let result: u64 = copies.iter().sum();
    Ok(result.to_string())
}

pub fn problem_01(puzzle_filename: &String) -> Result<String, String> {
    let document = std::fs::read_to_string(puzzle_filename).map_err(|e| e.to_string())?;
    let winnings = document
        .lines()
        .map(|row| count_wins(row).map(|n| score_wins(n)))
        .collect::<Result<Vec<u64>, String>>()?;
    let scratchcard_total: u64 = winnings.iter().sum();
    Ok(scratchcard_total.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_p1() {
        let test_file = String::from("../data/puzzle_04_test.txt");
        let result = problem_01(&test_file);
        assert_eq!(result, Ok(13.to_string()))
    }

    #[test]
    fn test_input_p2() {
        let test_file = String::from("../data/puzzle_04_test.txt");
        let result = problem_02(&test_file);
        assert_eq!(result, Ok(30.to_string()))
    }
}
