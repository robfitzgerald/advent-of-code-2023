// use rayon::prelude::*;

#[derive(Default, Debug)]
pub struct Accumulator {
    first: Option<u32>,
    last: Option<u32>,
}
impl Accumulator {
    pub fn add_number(&self, number: u32) -> Accumulator {
        Accumulator {
            first: self.first.or(Some(number)),
            last: Some(number),
        }
    }
    pub fn result(&self) -> Result<u32, String> {
        match (self.first, self.last) {
            (Some(a), Some(b)) => Ok((a * 10) + b),
            _ => Err(format!("incomplete result {:?}", self)),
        }
    }
}

pub fn match_word(remaining: &[u8]) -> Result<Option<u32>, String> {
    let length = remaining.len();
    if length < 3 {
        return Ok(None);
    }
    let window3 = std::str::from_utf8(&remaining[0..3]).map_err(|e| e.to_string())?;
    let result = match window3 {
        "one" => Some(1),
        "two" => Some(2),
        "six" => Some(6),
        _ => {
            if length < 4 {
                return Ok(None);
            }
            let window4 = std::str::from_utf8(&remaining[0..4]).map_err(|e| e.to_string())?;
            match window4 {
                "four" => Some(4),
                "five" => Some(5),
                "nine" => Some(9),
                _ => {
                    if length < 5 {
                        return Ok(None);
                    }
                    let window5 =
                        std::str::from_utf8(&remaining[0..5]).map_err(|e| e.to_string())?;
                    match window5 {
                        "three" => Some(3),
                        "seven" => Some(7),
                        "eight" => Some(8),
                        _ => None,
                    }
                }
            }
        }
    };
    Ok(result)
}

pub fn get_calibration_number(row: &str, use_words: bool) -> Result<u32, String> {
    fn _recurse(
        remaining: &[u8],
        acc: Accumulator,
        use_words: bool,
    ) -> Result<Accumulator, String> {
        match remaining.get(0) {
            None => Ok(acc),
            Some(next) => match (*next as char).to_digit(10) {
                Some(n) => _recurse(&remaining[1..], acc.add_number(n), use_words),
                None => {
                    if !use_words {
                        _recurse(&remaining[1..], acc, use_words)
                    } else {
                        let matched = match_word(remaining);
                        match matched {
                            Ok(None) => _recurse(&remaining[1..], acc, use_words),
                            Ok(Some(val)) => {
                                _recurse(&remaining[1..], acc.add_number(val), use_words)
                            }
                            Err(e) => Err(e),
                        }
                    }
                }
            },
        }
    }
    _recurse(row.as_bytes(), Accumulator::default(), use_words).and_then(|r| r.result())
}

pub fn run_day_01(puzzle_filename: &String, use_words: bool) -> Result<String, String> {
    let document = std::fs::read_to_string(puzzle_filename).map_err(|e| e.to_string())?;
    let result = document
        .lines()
        .map(|line| get_calibration_number(line, use_words))
        .fold(Ok(0 as u32), |a, b| match (a, b) {
            (Ok(x), Ok(y)) => Ok(x + y),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        });
    result.map(|value| value.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match_number_words() {
        let cases = vec![
            ("one", Some((3, 1))),
            ("two", Some((3, 2))),
            ("three", Some((5, 3))),
            ("four", Some((4, 4))),
            ("five", Some((4, 5))),
            ("six", Some((3, 6))),
            ("seven", Some((5, 7))),
            ("eight", Some((5, 8))),
            ("nine", Some((4, 9))),
            ("foola", None),
        ];
        for (word, expected_result) in cases.into_iter() {
            let suffix = "asdfjkl;";
            let suffixed = word.to_owned() + suffix;
            let suffixed_bytes = suffixed.as_str().as_bytes();
            if let Some((_len, value)) = expected_result {
                let result = match_word(word.as_bytes()).unwrap();
                assert_eq!(result, Some(value));
            }

            if let Some((len, _val)) = expected_result {
                let remove_result = suffixed_bytes[len as usize..].to_owned();
                assert_eq!(&remove_result, suffix.as_bytes());
            }
        }
    }
}
