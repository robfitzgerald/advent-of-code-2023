use itertools::Itertools;
use rayon::vec;
use std::hash::Hash;
use std::{collections::hash_map::DefaultHasher, fmt::Display};

#[derive(Eq, PartialEq, Debug)]
pub enum Entry {
    Number(char),
    Symbol(char),
    Space,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Entry::Number(n) => n.to_string(),
            Entry::Symbol(c) => c.to_string(),
            Entry::Space => String::from("."),
        };
        write!(f, "{}", content)
    }
}

impl Entry {
    pub fn new(c: char) -> Entry {
        match c.to_digit(10) {
            Some(_) => Entry::Number(c),
            None => match c {
                '.' => Entry::Space,
                _ => Entry::Symbol(c),
            },
        }
    }

    pub fn get_number(&self) -> Option<char> {
        match self {
            Entry::Number(c) => Some(*c),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        self.get_number().is_some()
    }

    pub fn is_asterisk(&self) -> bool {
        match self {
            Entry::Symbol('*') => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct EntryState {
    entry: Entry,
    is_part_number: bool,
    is_gear_ratio: bool,
}

impl Display for EntryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pn = if self.is_part_number {
            " part=√"
        } else {
            " part=x"
        };
        let gr = if self.is_gear_ratio {
            " gear=√"
        } else {
            " gear=x"
        };
        let guts = format!("{}{}{}", self.entry, pn, gr);
        write!(f, "({})", guts)
    }
}

impl EntryState {
    pub fn new(c: char) -> EntryState {
        let entry = Entry::new(c);
        EntryState {
            entry,
            is_part_number: false,
            is_gear_ratio: false,
        }
    }

    pub fn set_is_part_number(&mut self) {
        self.is_part_number = true;
    }

    pub fn set_is_gear_ratio(&mut self) {
        self.is_gear_ratio = true;
    }
}

pub fn get_from_table(table: &Vec<Vec<EntryState>>, x: i64, y: i64) -> Result<&EntryState, String> {
    let i_idx = x as usize;
    let j_idx = y as usize;
    let result_opt = table.get(i_idx).map(|row| row.get(j_idx));
    match result_opt {
        Some(Some(result)) => Ok(result),
        Some(None) => Err(format!("could not find y value of {} in table", y)),
        _ => Err(format!("could not find x value of {} in table", x)),
    }
}

pub fn adjacency_check(
    table: &Vec<Vec<EntryState>>,
    position: (i64, i64),
    check_op: &dyn Fn(&EntryState) -> bool,
) -> Result<Vec<(i64, i64)>, String> {
    let check = |i: i64, j: i64| -> bool {
        if i < 0 || table.len() as i64 <= i {
            return false;
        }
        let i_idx = i as usize;
        match table.get(i_idx) {
            None => false,
            Some(row) => {
                if j < 0 || row.len() as i64 <= j {
                    return false;
                }
                let j_idx = j as usize;
                match row.get(j_idx) {
                    Some(entry_state) => check_op(entry_state),
                    None => false,
                }
            }
        }
    };
    let (x, y) = position;

    let neighbors = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    let result = neighbors
        .iter()
        .filter(|(x, y)| check(*x, *y))
        .map(|p| p.to_owned())
        .collect::<Vec<_>>();

    // let result = agg_op(&check_result);

    Ok(result)
}

pub fn group_part_numbers(table: &Vec<Vec<EntryState>>) -> Vec<Vec<&EntryState>> {
    let mut solution: Vec<Vec<&EntryState>> = vec![];
    let mut grouped: Vec<&EntryState> = vec![];

    for row in table.iter() {
        for entry_state in row {
            match entry_state.entry {
                Entry::Number(_) => {
                    grouped.push(entry_state);
                }
                _ => {
                    if grouped.len() > 0 {
                        solution.push(grouped.clone());
                        grouped.clear();
                    }
                }
            }
        }
    }

    solution
}

pub fn group_parts_to_number(group: &Vec<&EntryState>) -> Result<Option<u64>, String> {
    let is_part_number = group.iter().any(|e| e.is_part_number);
    if !is_part_number {
        return Ok(None);
    }
    let s = group
        .iter()
        .flat_map(|e| e.entry.get_number())
        .collect::<String>();
    let parsed = s.parse::<u64>().map_err(|e| e.to_string())?;
    Ok(Some(parsed))
}

pub fn group_to_number(group: &Vec<&EntryState>) -> Result<Option<u64>, String> {
    let s = group
        .iter()
        .flat_map(|e| e.entry.get_number())
        .collect::<String>();
    let parsed = s.parse::<u64>().map_err(|e| e.to_string())?;
    Ok(Some(parsed))
}

pub fn find_group_positions_from_member(
    table: &Vec<Vec<EntryState>>,
    member_pos: (i64, i64),
) -> Result<Vec<(i64, i64)>, String> {
    let (member_x, member_y) = member_pos;
    let member_i = member_x as usize;
    let member_j = member_y as usize;

    let row = table
        .get(member_i)
        .ok_or(format!("invalid row {}", member_x))?;
    let prefix_inclusive = (0..member_j + 1)
        .rev()
        .take_while(|y| match row.get(*y) {
            Some(state) => state.entry.is_number(),
            None => false,
        })
        .map(|y| (member_i as i64, y as i64))
        .collect::<Vec<(i64, i64)>>()
        .into_iter()
        .rev();
    let suffix = (member_j + 1..row.len())
        .take_while(|y| match row.get(*y) {
            Some(state) => state.entry.is_number(),
            None => false,
        })
        .map(|y| (member_i as i64, y as i64))
        .collect::<Vec<(i64, i64)>>();
    let result = prefix_inclusive.chain(suffix).collect::<Vec<_>>();
    Ok(result)
}

pub fn find_group_from_member(
    table: &Vec<Vec<EntryState>>,
    member_pos: (i64, i64),
) -> Result<Vec<&EntryState>, String> {
    let (member_x, member_y) = member_pos;
    let member_i = member_x as usize;
    let member_j = member_y as usize;

    let row = table
        .get(member_i)
        .ok_or(format!("invalid row {}", member_x))?;
    let prefix_inclusive = (member_j..0)
        .take_while(|y| match row.get(*y) {
            Some(state) => state.entry.is_number(),
            None => false,
        })
        .flat_map(|y| row.get(y))
        .collect::<Vec<&EntryState>>()
        .into_iter()
        .rev();
    let suffix = (member_j + 1..row.len())
        .take_while(|y| match row.get(*y) {
            Some(state) => state.entry.is_number(),
            None => false,
        })
        .flat_map(|y| row.get(y))
        .collect::<Vec<&EntryState>>();
    let result = prefix_inclusive.chain(suffix).collect::<Vec<_>>();
    Ok(result)
}

pub fn problem_01(puzzle_filename: &String) -> Result<String, String> {
    let document = std::fs::read_to_string(puzzle_filename).map_err(|e| e.to_string())?;
    let mut table = document
        .lines()
        .map(|row_str| row_str.chars().map(EntryState::new).collect::<Vec<_>>())
        .collect::<Vec<Vec<EntryState>>>();

    // println!("{:?}", &table);
    let check_op = |entry_state: &EntryState| match entry_state.entry {
        Entry::Symbol(_) => true,
        _ => false,
    };

    // find any number tile that is part of a "part number", as in, is adjacent to a Entry::Symbol
    for i in 0..table.len() {
        let row_len = table[i].len();
        for j in 0..row_len {
            if table[i][j].entry.is_number() {
                let i_idx: i64 = i.try_into().map_err(|e| format!("{:?}", e))?;
                let j_idx: i64 = j.try_into().map_err(|e| format!("{:?}", e))?;
                let adjacencies = adjacency_check(&table, (i_idx, j_idx), &check_op)?;
                if adjacencies.len() > 0 {
                    table[i][j].set_is_part_number();
                }
            }
        }
    }

    let grouped = group_part_numbers(&table);
    let numbers_result = grouped
        .iter()
        .map(group_parts_to_number)
        .collect::<Result<Vec<Option<u64>>, String>>()?;
    let numbers: Vec<&u64> = numbers_result.iter().flatten().collect();

    let number: u64 = numbers.into_iter().sum();

    Ok(number.to_string())
}

pub fn equal_groups(a: &Vec<(i64, i64)>, b: &Vec<(i64, i64)>) -> bool {
    a.iter().zip(b).all(|(p1, p2)| p1 == p2)
}

pub fn find_unique_groups(groups: &Vec<Vec<(i64, i64)>>) -> Vec<Vec<(i64, i64)>> {
    let length = groups.len();
    let mut unique = vec![];
    let mut not_unique = vec![false; length];
    for g1 in 0..length {
        if not_unique[g1] {
            continue;
        }
        for g2 in g1..length {
            let equal = equal_groups(&groups[g1], &groups[g2]);
            if equal {
                not_unique[g2] = true;
            }
        }
        unique.push(groups[g1].to_vec());
    }
    unique
}

pub fn problem_02(puzzle_filename: &String) -> Result<String, String> {
    let document = std::fs::read_to_string(puzzle_filename).map_err(|e| e.to_string())?;
    let table = document
        .lines()
        .map(|row_str| row_str.chars().map(EntryState::new).collect::<Vec<_>>())
        .collect::<Vec<Vec<EntryState>>>();

    // println!("{:?}", &table);
    let check_op = |entry_state: &EntryState| entry_state.entry.is_number();

    // find any number tile that is part of a "gear ratio"
    let mut gear_ratios: Vec<u64> = vec![];
    for i in 0..table.len() {
        let row_len = table[i].len();
        for j in 0..row_len {
            let is_asterisk = table[i][j].entry.is_asterisk();
            if is_asterisk {
                let i_idx: i64 = i.try_into().map_err(|e| format!("{:?}", e))?;
                let j_idx: i64 = j.try_into().map_err(|e| format!("{:?}", e))?;
                let adjacencies = adjacency_check(&table, (i_idx, j_idx), &check_op)?;
                // todo: oh crap, we need to group numbers first here, then check that the
                // group count is exactly two.
                let groups: Vec<Vec<(i64, i64)>> = adjacencies
                    .iter()
                    .map(|adj| find_group_positions_from_member(&table, *adj))
                    .collect::<Result<Vec<Vec<(i64, i64)>>, String>>()?;

                let unique_groups = find_unique_groups(&groups);

                if unique_groups.len() == 2 {
                    let g_a = unique_groups[0]
                        .iter()
                        .map(|(x, y)| get_from_table(&table, *x, *y))
                        .collect::<Result<Vec<_>, String>>()?;
                    let g_b = unique_groups[1]
                        .iter()
                        .map(|(x, y)| get_from_table(&table, *x, *y))
                        .collect::<Result<Vec<_>, String>>()?;
                    let n_a =
                        group_to_number(&g_a)?.ok_or(format!("group not a number: {:?}", g_a))?;
                    let n_b =
                        group_to_number(&g_b)?.ok_or(format!("group not a number: {:?}", g_b))?;

                    gear_ratios.push(n_a * n_b)
                }
            }
        }
    }

    let number: u64 = gear_ratios.into_iter().sum();

    Ok(number.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let test_file = String::from("../data/puzzle_03_test.txt");
        let result = problem_01(&test_file);
        assert_eq!(result, Ok(4361.to_string()))
    }

    #[test]
    fn test_gears() {
        let test_file = String::from("../data/puzzle_03_gears.txt");
        let result = problem_02(&test_file);
        println!("{:?}", result);

        // find_group_positions_from_member gets weird.
        // the prefixes and suffixes it finds don't seem right.
    }

    #[test]
    fn test_unique() {
        let groups = vec![
            vec![(1, 2), (3, 4)],
            vec![(1, 2), (3, 4)],
            vec![(1, 2), (5, 6)],
        ];
        let result = find_unique_groups(&groups);
        assert_eq!(result.len(), 2);
        println!("{:?}", result);
    }
}
