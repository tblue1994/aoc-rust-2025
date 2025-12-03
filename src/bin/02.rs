use std::ops::Range;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let id_ranges = parse(input);
    let mut invalid_total = 0;
    for range in id_ranges {
        for x in range.start..range.end {
            let id = x.to_string();
            if (id.len() % 2) == 0 {
                if id[0..id.len() / 2] == id[id.len() / 2..] {
                    invalid_total += x;
                }
            }
        }
    }
    Some(invalid_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let id_ranges = parse(input);
    let mut invalid_total = 0;
    for range in id_ranges {
        for x in range.start..range.end {
            let mut invalid = false;
            let id = x.to_string();
            for sub_len in 1..(id.len() / 2) + 1 {
                if (id.len() % sub_len) == 0 {
                    let comp = &id[0..sub_len];
                    for i in 0..id.len() / sub_len {
                        if *comp != id[i * sub_len..i * sub_len + sub_len] {
                            invalid = false;
                            break;
                        }
                        invalid = true;
                    }
                }
                if invalid == true {
                    break;
                }
            }
            if invalid == true {
                invalid_total += x;
            }
        }
    }
    Some(invalid_total)
}

pub fn parse(input: &str) -> Vec<Range<u64>> {
    let groups = input.lines().next().unwrap().split([',']);
    let mut ids: Vec<Range<u64>> = Vec::new();
    for pair in groups.into_iter() {
        let mut ends = pair.split(['-']);
        let start = ends.next().unwrap().parse::<u64>().unwrap();
        let end = ends.next().unwrap().parse::<u64>().unwrap();
        ids.push(Range {
            start: start,
            end: end + 1,
        });
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
