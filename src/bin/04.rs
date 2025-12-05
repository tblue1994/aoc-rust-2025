use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let roll_loc_vec = parse(input);
    let roll_locations = roll_loc_vec.as_slice();
    let mut roll_count = 0;
    for loc in roll_locations {
        let adjacent_locs = get_adjacent(*loc);
        let mut adj_count = 0;
        for adj in adjacent_locs {
            if roll_locations.contains(&adj) {
                adj_count += 1;
            }
            if adj_count >= 4 {
                break;
            }
        }
        if adj_count < 4 {
            roll_count += 1;
        }
    }

    Some(roll_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let roll_loc_vec = parse(input);
    let roll_locations = roll_loc_vec.as_slice();
    let mut moved_rolls: HashSet<(isize, isize)> = HashSet::new();
    let mut change = true;
    while change {
        change = false;
        for loc in roll_locations {
            if moved_rolls.contains(loc) {
                continue;
            }
            let adjacent_locs = get_adjacent(*loc);
            let mut adj_count = 0;
            for adj in adjacent_locs {
                if !moved_rolls.contains(&adj) && roll_locations.contains(&adj) {
                    adj_count += 1;
                }
                if adj_count >= 4 {
                    break;
                }
            }
            if adj_count < 4 {
                moved_rolls.insert(*loc);
                change = true
            }
        }
    }

    Some(moved_rolls.len() as u64)
}

pub fn parse(input: &str) -> Vec<(isize, isize)> {
    let lines = input.lines();
    let mut grid: Vec<(isize, isize)> = Vec::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                grid.push((x as isize, y as isize));
            }
        }
    }
    grid
}

pub fn get_adjacent(loc: (isize, isize)) -> Vec<(isize, isize)> {
    return Vec::from([
        (loc.0 + 1, loc.1),
        (loc.0 - 1, loc.1),
        (loc.0, loc.1 + 1),
        (loc.0, loc.1 - 1),
        (loc.0 + 1, loc.1 + 1),
        (loc.0 - 1, loc.1 - 1),
        (loc.0 - 1, loc.1 + 1),
        (loc.0 + 1, loc.1 - 1),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
