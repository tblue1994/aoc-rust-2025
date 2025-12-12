use std::{
    cmp::{max, min},
    collections::HashMap,
};

use geo::Contains;
use geo_types::{LineString, Polygon, point};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let locations = parse(input);
    let mut red_tile_areas: Vec<isize> = vec![];
    for i in 0..locations.len() - 1 {
        for j in i + 1..locations.len() {
            red_tile_areas.push(find_area(locations[i], locations[j]));
        }
    }
    let x = *red_tile_areas.iter().max().unwrap();

    Some(x)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut locations = parse(input);
    let mut cache: HashMap<(isize, isize), bool> = HashMap::new();
    for x in locations.clone() {
        cache.insert(x, true);
    }
    locations.push(locations[0]);

    let polygon = Polygon::<isize>::new(LineString::from(locations.clone()), vec![]);
    let mut red_tile_areas: Vec<(isize, (isize, isize), (isize, isize))> = vec![];
    for i in 0..locations.len() - 1 {
        for j in i + 1..locations.len() {
            red_tile_areas.push((
                find_area(locations[i], locations[j]),
                locations[i],
                locations[j],
            ));
        }
    }

    red_tile_areas.sort_by(|a, b| b.0.cmp(&a.0));

    for (area, p1, p2) in red_tile_areas {
        println!("{}", area);
        if all_contained(p1, p2, &polygon, &mut cache) {
            return Some(area);
        }
    }

    None
}

pub fn find_area(l1: (isize, isize), l2: (isize, isize)) -> isize {
    ((l1.0 - l2.0).abs() + 1) * ((l1.1 - l2.1).abs() + 1)
}

pub fn all_contained(
    l1: (isize, isize),
    l2: (isize, isize),
    polygon: &Polygon<isize>,
    cache: &mut HashMap<(isize, isize), bool>,
) -> bool {
    let min_x = min(l1.0, l2.0);
    let max_x = max(l1.0, l2.0);
    let min_y = min(l1.1, l2.1);
    let max_y = max(l1.1, l2.1);

    for x in min_x..max_x + 1 {
        let bot_point = &point!(x:x, y:min_y);
        let top_point = &point!(x:x, y:max_y);

        if !cache.contains_key(&(x, min_y)) {
            cache.insert(
                (x, min_y),
                polygon.exterior().contains(bot_point) || polygon.contains(bot_point),
            );
        }
        if !cache.contains_key(&(x, max_y)) {
            cache.insert(
                (x, max_y),
                polygon.exterior().contains(top_point) || polygon.contains(top_point),
            );
        }
        if !(*cache.get(&(x, min_y)).unwrap() && *cache.get(&(x, max_y)).unwrap()) {
            return false;
        }
    }
    for y in min_y..max_y + 1 {
        let bot_point = &point!(x:min_x, y:y);
        let top_point = &point!(x:max_x, y:y);
        if !cache.contains_key(&(min_x, y)) {
            cache.insert(
                (min_x, y),
                polygon.exterior().contains(bot_point) || polygon.contains(bot_point),
            );
        }
        if !cache.contains_key(&(max_x, y)) {
            cache.insert(
                (max_x, y),
                polygon.exterior().contains(top_point) || polygon.contains(top_point),
            );
        }
        if !(*cache.get(&(min_x, y)).unwrap() && *cache.get(&(max_x, y)).unwrap()) {
            return false;
        }
    }
    true
}

pub fn parse(input: &str) -> Vec<(isize, isize)> {
    let mut locations: Vec<(isize, isize)> = vec![];
    for line in input.lines() {
        let mut split = line.split(',');
        let loc = (
            split.next().unwrap().parse::<isize>().unwrap(),
            split.next().unwrap().parse::<isize>().unwrap(),
        );
        locations.push(loc);
    }

    locations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
