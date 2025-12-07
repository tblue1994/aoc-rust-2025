use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let (start, hash) = parse(input);
    let mut split_count = 0;
    let mut current_hash: HashSet<usize> = HashSet::from([start]);
    for i in 1..hash.len() + 1 {
        let mut iteration_hash: HashSet<usize> = HashSet::new();
        let line_set = hash.get(&i).unwrap();
        if line_set.is_empty() {
            continue;
        }
        for x in current_hash {
            if line_set.contains(&x) {
                split_count += 1;
                iteration_hash.insert(x + 1);
                iteration_hash.insert(x - 1);
            } else {
                iteration_hash.insert(x);
            }
        }
        current_hash = iteration_hash;
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start, hash) = parse(input);
    let mut calculated: HashMap<(usize, usize), u64> = HashMap::new();
    return Some(recursion_hell((start, 1), &hash, &mut calculated));
}

pub fn parse(input: &str) -> (usize, HashMap<usize, HashSet<usize>>) {
    let lines = input.lines();
    let mut start = 0;
    let mut hash: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, line) in lines.enumerate() {
        if i == 0 {
            start = line.chars().position(|c| c == 'S').unwrap()
        } else {
            hash.insert(i, HashSet::from_iter(line.match_indices("^").map(|x| x.0)));
        }
    }

    (start, hash)
}

pub fn recursion_hell(
    location: (usize, usize),
    map: &HashMap<usize, HashSet<usize>>,
    already_calculated: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if already_calculated.contains_key(&location) {
        return *already_calculated.get(&location).unwrap();
    }

    let line_set = map.get(&location.1).unwrap();
    let value;
    if location.1 == map.len() {
        value = 1;
    } else if line_set.contains(&location.0) {
        value = recursion_hell((location.0 + 1, location.1 + 1), map, already_calculated)
            + recursion_hell((location.0 - 1, location.1 + 1), map, already_calculated);
    } else {
        value = recursion_hell((location.0, location.1 + 1), map, already_calculated);
    }
    already_calculated.insert(location, value);
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
