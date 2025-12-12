use good_lp::*;
use std::collections::{HashMap, VecDeque};

use regex::Regex;

advent_of_code::solution!(10);

pub struct LightsButtonsJoltage {
    desired_state: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let line_values = parse(input);
    let mut total_presses = 0;
    for line_value in line_values {
        total_presses += solve_1_for_line(
            &line_value.desired_state,
            &line_value
                .buttons
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<&[usize]>>(),
        );
    }
    Some(total_presses)
}

pub fn part_two(input: &str) -> Option<u64> {
    let line_values = parse(input);
    let mut total_presses = 0;
    for line_value in line_values {
        total_presses += solve_2_goodlp(
            &line_value.joltage,
            &line_value
                .buttons
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<&[usize]>>(),
        );
    }
    Some(total_presses)
}

pub fn solve_1_for_line(desired: &[bool], buttons: &[&[usize]]) -> u64 {
    let desired_string: String = desired
        .iter()
        .map(|x| if *x == true { '#' } else { '.' })
        .collect();
    let start: String = desired.iter().map(|_| '.').collect();
    let mut visited: HashMap<String, u64> = HashMap::new();
    visited.insert(start.clone(), 0);
    let mut queue: VecDeque<(String, &[usize], u64)> =
        buttons.iter().map(|x| (start.clone(), *x, 1)).collect();
    let mut current;
    while queue.len() > 0 {
        let (val, button, count) = queue.pop_front().unwrap();
        current = val.clone();
        for x in button {
            if val.chars().nth(*x).unwrap() == '#' {
                current.replace_range(*x..*x + 1, ".");
            } else {
                current.replace_range(*x..*x + 1, "#");
            }
        }

        if visited.get(&current).is_none_or(|x| *x > count) {
            visited.insert(current.clone(), count);
            for button in buttons {
                queue.push_back((current.clone(), &button, count + 1));
            }
        }
    }

    *visited.get(&desired_string).unwrap()
}

pub fn solve_2_goodlp(desired: &[u64], buttons: &[&[usize]]) -> u64 {
    let mut vars = variables!();
    let presses: Vec<Variable> = vars.add_all(vec![variable().min(0).integer(); buttons.len()]);
    let exp: Expression = presses.iter().sum();
    let mut problem = vars.minimise(exp).using(default_solver);
    for (i, x) in desired.iter().enumerate() {
        let mut expr = Expression::from(0.0);

        for (btn_idx, relevant_idxs) in buttons.iter().enumerate() {
            // if button is relevant, add its press variable to the constraint
            if relevant_idxs.contains(&i) {
                expr += presses[btn_idx];
            }
        }

        // sum of relevant presses == target joltage
        problem.add_constraint(expr.eq(*x as f64));
    }

    let solution = problem.solve().unwrap();

    presses
        .iter()
        .map(|v| solution.value(*v).round() as u64)
        .sum()
}

pub fn parse(input: &str) -> Vec<LightsButtonsJoltage> {
    let mut return_type: Vec<LightsButtonsJoltage> = vec![];
    let re = Regex::new(r"\[(.*)\] (.*) \{(.*)}").unwrap();
    for line in input.lines() {
        let mut line_value = LightsButtonsJoltage {
            desired_state: vec![],
            buttons: vec![],
            joltage: vec![],
        };
        let caps = re.captures(line).unwrap();
        let desired_state = caps.get(1).map_or("", |m| m.as_str());
        let buttons = caps.get(2).map_or("", |m| m.as_str());
        let joltage = caps.get(3).map_or("", |m| m.as_str());
        line_value.desired_state = desired_state.chars().map(|x| x == '#').collect();
        let button_split = buttons.split_whitespace();
        for split in button_split {
            let numbers_split = split[1..split.len() - 1].split(',');
            line_value
                .buttons
                .push(numbers_split.map(|x| x.parse().unwrap()).collect());
        }

        for jsplit in joltage.split(',') {
            line_value.joltage.push(jsplit.parse().unwrap());
        }
        return_type.push(line_value);
    }
    return_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
