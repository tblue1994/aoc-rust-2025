use std::f64;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    sub_part_one(input, 1000)
}

pub fn sub_part_one(input: &str, connections: usize) -> Option<u64> {
    let junctions = parse(input);
    let mut junction_distances: Vec<(f64, ((f64, f64, f64), (f64, f64, f64)))> = vec![];
    for i in 0..junctions.len() - 1 {
        for j in i + 1..junctions.len() {
            junction_distances.push((
                euclidean_distance_3d(junctions[i], junctions[j]),
                (junctions[i], junctions[j]),
            ));
        }
    }
    junction_distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut junction_hookups = junctions
        .iter()
        .map(|x| vec![*x])
        .collect::<Vec<Vec<(f64, f64, f64)>>>();
    let mut index = 0;
    while index < connections {
        let (p1, p2) = junction_distances[index].1;
        let p1_index = junction_hookups
            .iter()
            .position(|r| r.contains(&p1))
            .unwrap();
        let p2_index = junction_hookups
            .iter()
            .position(|r| r.contains(&p2))
            .unwrap();

        if p1_index != p2_index {
            let clone = junction_hookups[p2_index].clone();
            junction_hookups[p1_index].extend(clone);
            junction_hookups.remove(p2_index);
        }
        index += 1;
    }
    junction_hookups.sort_by(|a, b| b.len().cmp(&a.len()));

    Some((junction_hookups[0].len() * junction_hookups[1].len() * junction_hookups[2].len()) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions = parse(input);
    let mut junction_distances: Vec<(f64, ((f64, f64, f64), (f64, f64, f64)))> = vec![];
    for i in 0..junctions.len() - 1 {
        for j in i + 1..junctions.len() {
            junction_distances.push((
                euclidean_distance_3d(junctions[i], junctions[j]),
                (junctions[i], junctions[j]),
            ));
        }
    }
    junction_distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut junction_hookups = junctions
        .iter()
        .map(|x| vec![*x])
        .collect::<Vec<Vec<(f64, f64, f64)>>>();
    let mut index = 0;
    let mut found = false;
    let mut x_distance_last = 0;
    while !found {
        let (p1, p2) = junction_distances[index].1;
        let p1_index = junction_hookups
            .iter()
            .position(|r| r.contains(&p1))
            .unwrap();
        let p2_index = junction_hookups
            .iter()
            .position(|r| r.contains(&p2))
            .unwrap();

        if p1_index != p2_index {
            if junction_hookups.len() == 2 {
                found = true;
                x_distance_last = (p1.0 * p2.0) as u64
            } else {
                let clone = junction_hookups[p2_index].clone();
                junction_hookups[p1_index].extend(clone);
                junction_hookups.remove(p2_index);
            }
        }
        index += 1;
    }
    junction_hookups.sort_by(|a, b| b.len().cmp(&a.len()));

    Some(x_distance_last)
}

pub fn parse(input: &str) -> Vec<(f64, f64, f64)> {
    let mut junctions: Vec<(f64, f64, f64)> = vec![];
    for line in input.lines() {
        let mut split = line.split(',');
        let junction = (
            split.next().unwrap().parse::<f64>().unwrap(),
            split.next().unwrap().parse::<f64>().unwrap(),
            split.next().unwrap().parse::<f64>().unwrap(),
        );
        junctions.push(junction);
    }

    junctions
}

fn euclidean_distance_3d(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let dz = p2.2 - p1.2;

    (dx * dx + dy * dy + dz * dz).sqrt().abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = sub_part_one(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
