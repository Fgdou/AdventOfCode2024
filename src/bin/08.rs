use std::collections::{HashMap, HashSet};

use itertools::iproduct;
use vectorlib::math::vector2d_module::Vector2d;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Input {
    map_size: Vector2d<i32>,
    antenas: HashMap<char, Vec<Vector2d<i32>>>,
}

fn parse(input: &str) -> Input {
    let mut map_size = None;
    let antenas = input.split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            map_size = match map_size {
                None => Some(Vector2d::new(line.len() as i32, y as i32 + 1)),
                Some(_) => Some(Vector2d::new(line.len() as i32, y as i32 + 1)),
            };

            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (c, Vector2d::new(x as i32, y as i32)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, (char, pos)| {
            acc.entry(char).or_insert(Vec::new()).push(pos);
            acc
        });
    Input{
        map_size: map_size.unwrap(),
        antenas,
    }
}

fn get_antinode(pos1: &Vector2d<i32>, pos2: &Vector2d<i32>, map_size: &Vector2d<i32>) -> Option<Vector2d<i32>> {
    let diff = *pos2 - *pos1;
    let antinode = *pos2 + diff;

    if antinode.x < 0 || antinode.y < 0 || antinode.x >= map_size.x || antinode.y >= map_size.y {
        None
    } else {
        Some(antinode)
    }
}
fn get_antinode2(pos1: &Vector2d<i32>, pos2: &Vector2d<i32>, map_size: &Vector2d<i32>) -> Vec<Vector2d<i32>> {
    let diff = *pos2 - *pos1;

    let mut res = vec!();
    let mut i = 0;

    loop {
        let antinode = *pos2 + diff*i;

        if antinode.x < 0 || antinode.y < 0 || antinode.x >= map_size.x || antinode.y >= map_size.y {
            break;
        } else {
            res.push(antinode);
        }
        i += 1;
    }

    res
}

fn get_antinodes(antenas: &Vec<Vector2d<i32>>, map_size: &Vector2d<i32>) -> Vec<Vector2d<i32>> {
    iproduct!(antenas, antenas)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| get_antinode(a, b, map_size))
        .flatten()
        .collect()
}
fn get_antinodes2(antenas: &Vec<Vector2d<i32>>, map_size: &Vector2d<i32>) -> Vec<Vector2d<i32>> {
    iproduct!(antenas, antenas)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| get_antinode2(a, b, map_size))
        .flatten()
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let antinodes = input.antenas.values()
        .map(|antenas| {
            get_antinodes(antenas, &input.map_size)
        })
        .flatten()
        .fold(HashSet::new(), |mut acc, vec| {
            acc.insert((vec.x, vec.y));
            acc
        });
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let antinodes = input.antenas.values()
        .map(|antenas| {
            get_antinodes2(antenas, &input.map_size)
        })
        .flatten()
        .fold(HashSet::new(), |mut acc, vec| {
            acc.insert((vec.x, vec.y));
            acc
        });
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_antinode() {
        assert_eq!(
            get_antinode(&Vector2d::new(3, 3), &Vector2d::new(5, 5), &Vector2d::new(10, 10)),
            Some(Vector2d::new(7, 7))
        )
    }
    #[test]
    fn test_antinodes() {
        assert_eq!(
            get_antinodes(&vec!(
                Vector2d::new(3, 3),
                Vector2d::new(5, 5),
            ), &Vector2d::new(10, 10)),
            vec!(
                Vector2d::new(7, 7),
                Vector2d::new(1, 1),
            )
        );
        assert_eq!(
            get_antinodes(&vec!(
                Vector2d::new(4, 3),
                Vector2d::new(5, 4),
                Vector2d::new(8, 5),
            ), &Vector2d::new(10, 10)).len(),
            4
        );
    }
}
