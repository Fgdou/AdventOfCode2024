use std::collections::HashSet;

use itertools::iproduct;
use vectorlib::math::vector2d_module::Vector2d;

advent_of_code::solution!(10);

type Map = Vec<Vec<u32>>;

fn parse(input: &str) -> Map {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()  
        }).collect()
}

type Possibilities = HashSet<(i32, i32)>;
fn next(requested_number: u32, pos: &Vector2d<i32>, map: &Map, map_size: &Vector2d<i32>) -> Option<Possibilities> {
    let n = *map.get(pos.y as usize)?.get(pos.x as usize)?;
    if requested_number != n {
        return None;
    }
    if n == 9 {
        return Some(HashSet::from([(pos.x, pos.y)]));
    }

    let dirs = vec!(
        Vector2d::new(-1, 0),
        Vector2d::new(1, 0),
        Vector2d::new(0, -1),
        Vector2d::new(0, 1),
    );

    let res = dirs.into_iter().filter_map(|dir| {
        let nextpos = *pos + dir;
        next(requested_number+1, &nextpos, map, map_size)
    }).flatten().collect::<HashSet<_>>();

    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

fn find_paths(map: &Map) -> Vec<(Vector2d<i32>, Possibilities)> {
    let map_size = Vector2d::new(
        map.get(0).unwrap().len() as i32, 
        map.len() as i32
    );

    iproduct!(0..map_size.x, 0..map_size.y)
        .map(|(x, y)| Vector2d::new(x, y))
        .filter_map(|pos| {
            Some((pos, next(0, &pos, map, &map_size)?))
        })
        .collect()
}

fn next2(requested_number: u32, pos: &Vector2d<i32>, map: &Map, map_size: &Vector2d<i32>) -> Option<usize> {
    let n = *map.get(pos.y as usize)?.get(pos.x as usize)?;
    if requested_number != n {
        return None;
    }
    if n == 9 {
        return Some(1);
    }

    let dirs = vec!(
        Vector2d::new(-1, 0),
        Vector2d::new(1, 0),
        Vector2d::new(0, -1),
        Vector2d::new(0, 1),
    );

    let res: usize = dirs.into_iter().filter_map(|dir| {
        let nextpos = *pos + dir;
        next2(requested_number+1, &nextpos, map, map_size)
    }).sum();

    if res == 0 {
        None
    } else {
        Some(res)
    }
}

fn find_paths2(map: &Map) -> usize {
    let map_size = Vector2d::new(
        map.get(0).unwrap().len() as i32, 
        map.len() as i32
    );

    iproduct!(0..map_size.x, 0..map_size.y)
        .map(|(x, y)| Vector2d::new(x, y))
        .filter_map(|pos| {
            next2(0, &pos, map, &map_size)
        })
        .sum()
}


pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let paths = find_paths(&input);
    let count = paths.iter().map(|(_, vec)| vec.len()).sum();
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let count = find_paths2(&input);
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
