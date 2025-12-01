advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let twists = parse(input);
    let mut password = 0;
    let mut current_value = 50;
    for twist in twists {
        current_value += twist;
        current_value %= 100;
        if current_value == 0 {
            password += 1;
        }
    }

    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let twists = parse(input);
    let mut password = 0;
    let mut current_value = 50;
    for twist in twists {
        for _ in 0..twist.abs() {
            if twist < 0 {
                current_value -= 1;
            } else {
                current_value += 1;
            }
            current_value %= 100;
            if current_value == 0 {
                password += 1;
            }
        }
    }

    Some(password)
}

pub fn parse(input: &str) -> Vec<isize> {
    let lines = input.lines();
    let mut list1: Vec<isize> = Vec::new();
    for line in lines {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let mut distance = chars.as_str().parse::<isize>().unwrap();
        if direction == 'L' {
            distance *= -1;
        }
        list1.push(distance);
    }
    list1
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
        assert_eq!(result, Some(6));
    }
}
