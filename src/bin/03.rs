advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let battery_lines = parse(input);
    let mut total_joltage = 0;
    for battery in battery_lines {
        total_joltage += find_max(battery[0], battery[1], &battery[2..])
    }

    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let battery_lines = parse(input);
    let mut total_joltage = 0;
    for battery in battery_lines {
        total_joltage += find_max_long(&battery[0..12], &battery[12..])
    }

    Some(total_joltage)
}

pub fn find_max(current_first_digit: u64, current_second_digit: u64, rest_battery: &[u64]) -> u64 {
    if rest_battery.len() == 0 {
        return current_first_digit * 10 + current_second_digit;
    }
    let compare = rest_battery[0];
    if current_second_digit > current_first_digit {
        return find_max(current_second_digit, compare, &rest_battery[1..]);
    }
    if compare > current_second_digit {
        return find_max(current_first_digit, compare, &rest_battery[1..]);
    }
    return find_max(
        current_first_digit,
        current_second_digit,
        &rest_battery[1..],
    );
}

pub fn find_max_long(current: &[u64], rest_battery: &[u64]) -> u64 {
    if rest_battery.len() == 0 {
        let mut total = 0;
        for (i, digit) in current.into_iter().rev().enumerate() {
            total += (*digit) * (10_u64.pow(i as u32));
        }

        return total;
    }
    let compare = rest_battery[0];

    for d in 0..current.len() {
        if d == current.len() - 1 {
            if current[d] < compare {
                return find_max_long(
                    &mut [&current[..d], &[compare]].concat(),
                    &rest_battery[1..],
                );
            }
        } else if current[d] < current[d + 1] {
            return find_max_long(
                &mut [&current[..d], &current[d + 1..], &[compare]].concat(),
                &rest_battery[1..],
            );
        }
    }

    return find_max_long(current, &rest_battery[1..]);
}

pub fn parse(input: &str) -> Vec<Vec<u64>> {
    let lines = input.lines();
    let mut battery_lines: Vec<Vec<u64>> = Vec::new();
    for line in lines.into_iter() {
        let mut battery: Vec<u64> = Vec::new();
        for c in line.chars() {
            battery.push(c.to_digit(10).map(u64::from).unwrap());
        }
        battery_lines.push(battery);
    }
    battery_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
