use std::collections::HashSet;

use vectorlib::math::vector2d_module::Vector2d;

advent_of_code::solution!(12);

type Input = Vec<Vec<char>>;
fn parse(input: &str) -> Input {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn count_sides(pos: &Vector2d<i32>, map: &Input) -> usize {
    let c = map.get(pos.y as usize).unwrap().get(pos.x as usize).unwrap();

    let dirs = [
        Vector2d::new(1, 0),
        Vector2d::new(-1, 0),
        Vector2d::new(0, 1),
        Vector2d::new(0, -1),
    ];

    dirs.into_iter().map(|dir| pos.clone() + dir)
        .map(|pos| {
            let other_c = map.get(pos.y as usize).and_then(|line| line.get(pos.x as usize));
            
            match other_c {
                None => true,
                Some(other_c) => other_c != c
            }
        })
        .filter(|res| *res)
        .count()
}

#[derive(Debug, Clone, PartialEq)]
struct Group {
    c: char,
    poses: Vec<Vector2d<i32>>,
}
fn get_group(pos: &Vector2d<i32>, map: &Input, visited: &mut HashSet<(i32, i32)>) -> Option<Group> {
    if visited.contains(&(pos.x, pos.y)) {
        return None;
    }
    
    let c = map.get(pos.y as usize).unwrap().get(pos.x as usize).unwrap();
    let mut stack = vec!(*pos);
    let mut in_group = Vec::new();

    while let Some(pos) = stack.pop() {
        if visited.contains(&(pos.x, pos.y)) {
            continue;
        }

        let other_c = map.get(pos.y as usize).and_then(|line| line.get(pos.x as usize));
        let other_c = match other_c {
            None => continue,
            Some(c) => c,
        };

        if other_c != c {
            continue;
        }

        visited.insert((pos.x, pos.y));
        in_group.push(pos);

        let dirs = [
            Vector2d::new(-1, 0),
            Vector2d::new(1, 0),
            Vector2d::new(0, 1),
            Vector2d::new(0, -1),
        ];

        for d in dirs {
            let pos = d + pos;
            stack.push(pos);
        }
    }

    Some(Group {
        c: *c,
        poses: in_group
    })
}

fn get_groups(map: &Input) -> Vec<Group> {
    let mut visited = HashSet::new();
    map.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().filter_map(|(x, c)| {
            let pos = Vector2d::new(x as i32, y as i32);
            get_group(&pos, map, &mut visited)
        }).collect::<Vec<_>>()
    }).flatten().collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    let groups = get_groups(&map);

    let sum = groups.iter().map(|group| {
        let edges: usize = group.poses.iter()
            .map(|pos| count_sides(pos, &map))
            .sum();
        let area = group.poses.len();
        edges * area
    }).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(140));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(1930));
    }
    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_get_group() {
        assert_eq!(
            get_group(&Vector2d::new(0, 0), &vec!(
                vec!('a')
            ), &mut HashSet::new()),
            Some(Group{
                c: 'a',
                poses: vec!(Vector2d::new(0, 0))
            })
        );
        assert_eq!(
            get_group(&Vector2d::new(0, 0), &vec!(
                vec!('a', 'b')
            ), &mut HashSet::new()),
            Some(Group{
                c: 'a',
                poses: vec!(Vector2d::new(0, 0))
            })
        );
        assert_eq!(
            get_group(&Vector2d::new(0, 0), &vec!(
                vec!('a', 'a')
            ), &mut HashSet::new()),
            Some(Group{
                c: 'a',
                poses: vec!(
                    Vector2d::new(0, 0),
                    Vector2d::new(1, 0),
                )
            })
        );
    }

    #[test]
    fn test_edges() {
        assert_eq!(
            count_sides(&Vector2d::new(0, 0), &vec!(
                vec!('a')
            )),
            4
        );
        assert_eq!(
            count_sides(&Vector2d::new(0, 0), &vec!(
                vec!('a', 'b')
            )),
            4
        );
        assert_eq!(
            count_sides(&Vector2d::new(0, 0), &vec!(
                vec!('a', 'a')
            )),
            3
        );
    }
}
