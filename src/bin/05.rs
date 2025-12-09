use std::{
    cmp::{max, min},
    ops::Range,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse(input);
    let range_slice = ranges.as_slice();
    let mut fresh = 0;
    for ing in ingredients {
        for range in range_slice {
            if range.contains(&ing) {
                fresh += 1;
                break;
            }
        }
    }

    Some(fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse(input);
    let mut final_ranges = ranges;
    let mut change = true;
    while change {
        change = false;
        let length = final_ranges.len();
        for i in 0..length - 1 {
            for j in i + 1..length {
                let (overlap, start, end) =
                    find_overlap(final_ranges[i].clone(), final_ranges[j].clone());
                if overlap {
                    final_ranges.remove(j);
                    final_ranges.remove(i);
                    final_ranges.push(Range { start, end });
                    change = true;
                    break;
                }
            }
            if change {
                break;
            }
        }
    }
    let mut total = 0;
    for range in final_ranges {
        total += range.end - range.start;
    }

    Some(total)
}

pub fn find_overlap(range1: Range<u64>, range2: Range<u64>) -> (bool, u64, u64) {
    //range 1 inside range 2

    //range 2 inside range 1

    //range 1 end overlaps with range 2 start

    //range 2 end overlaps with range 1 start
    if (range2.end >= range1.start && range2.start < range1.end)
        || (range1.end >= range2.start && range1.start < range2.end)
    {
        (
            true,
            min(range1.start, range2.start),
            max(range1.end, range2.end),
        )
    } else {
        (false, 0, 0)
    }
}

pub fn parse(input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let lines = input.lines();
    let mut fresh_ranges: Vec<Range<u64>> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    for line in lines {
        if line.contains('-') {
            let mut splits = line.split('-');
            let first = splits.next().unwrap().parse::<u64>().unwrap();
            let end = splits.next().unwrap().parse::<u64>().unwrap();
            fresh_ranges.push(Range {
                start: first,
                end: end + 1,
            });
        } else if line.len() > 0 {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }
    (fresh_ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
