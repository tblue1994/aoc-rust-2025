advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (problems, signs) = parse(input);
    let mut total_total = 0;
    for i in 0..signs.len() {
        let sign = signs[i];
        let mut problem_total = 0;
        if sign == "*" {
            problem_total = 1;
        }

        for line in &problems {
            if sign == "*" {
                problem_total *= line[i];
                continue;
            }
            problem_total += line[i]
        }
        total_total += problem_total
    }
    Some(total_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut sign_line: String = "".to_string();
    let mut problem_lines: Vec<&str> = vec![];
    for line in lines {
        if line.contains("*") {
            sign_line = line.to_string();
            continue;
        }
        problem_lines.push(line);
    }

    let mut total_total = 0;
    let mut problem_total = 0;
    let mut is_multiplication = false;
    for (i, c) in sign_line.chars().enumerate() {
        if c == '*' {
            is_multiplication = true;
            problem_total = 1;
        }
        let mut current_number_string = "".to_string();
        for line in &problem_lines {
            current_number_string.push(line.chars().nth(i).unwrap());
        }
        let value = current_number_string.trim().parse::<u64>();
        if value.is_ok() {
            //continue
            if is_multiplication {
                problem_total *= value.unwrap()
            } else {
                problem_total += value.unwrap()
            }
        } else {
            //push value and reset
            total_total += problem_total;
            problem_total = 0;
            is_multiplication = false
        }
    }
    //add final
    total_total += problem_total;
    Some(total_total)
}

pub fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let mut problems: Vec<Vec<u64>> = vec![];
    let mut signs: Vec<&str> = vec![];
    let lines = input.lines();
    for line in lines {
        if line.contains("*") {
            signs = line.split_whitespace().collect();
            continue;
        }
        problems.push(
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    (problems, signs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
