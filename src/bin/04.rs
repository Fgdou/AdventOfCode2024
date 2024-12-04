use itertools::iproduct;
use vectorlib::math::vector2d_module::Vector2d;

advent_of_code::solution!(4);

type Map = Vec<Vec<char>>;

fn parse_input(input: &str) -> Map {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line|
            line.chars().collect()
        )
        .collect()
}

fn count_word(map: &Map, pos: &Vector2d<i32>, word: &str) -> usize {
    let directions: Vec<Vector2d<i32>> = iproduct!(-1..=1, -1..=1)
        .filter(|(n1, n2)| !(*n1 == 0 && *n2 == 0))
        .map(|(n1, n2)| Vector2d::new(n1, n2))
        .collect();

    directions.into_iter().filter(|direction| {
            word.char_indices().all(|(index, c)| {
                let new_pos = *pos + *direction * index as i32;

                if new_pos.x < 0 || new_pos.y < 0 {
                    return false;
                }
                if new_pos.y >= map.len() as i32 || new_pos.x >= map.get(0).unwrap().len() as i32 {
                    return false;
                }

                let char = map.get(new_pos.y as usize).unwrap().get(new_pos.x as usize).unwrap();
                *char == c
            })
        }).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse_input(input);

    let y = input.len() as i32;
    let x = input.get(0).unwrap().len() as i32;

    let count = iproduct!(0..y, 0..x).map(|(x, y)| Vector2d::new(x, y))
        .map(|pos| count_word(&input, &pos, "XMAS"))
        .sum();

    Some(count)
}

fn test_possibility(map: &Map, pos: &Vector2d<i32>, possibility: &(String, Vec<Vector2d<i32>>)) -> bool {
    possibility.0.chars().zip(possibility.1.clone())
        .all(|(c, p)| {
            let pos = *pos + p;

            if pos.x < 0 || pos.y < 0 || pos.x >= map.get(0).unwrap().len() as i32 || pos.y >= map.len() as i32 {
                return false;
            }

            *map.get(pos.y as usize).unwrap().get(pos.x as usize).unwrap() == c
        })
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse_input(input);

    let word = "MAS";
    let words: Vec<String> = vec!(word.chars().rev().collect(), word.to_string());
    let all_poses: Vec<Vec<_>> = vec!(
        (0..3).map(|i| Vector2d::new(i, i)).collect(),
        (0..3).map(|i| Vector2d::new(i, 2-i)).collect()
    );

    let full_possibilities: Vec<_> = iproduct!(words, all_poses).collect();

    let result = input.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().filter(|(x, _)| {
            let pos = Vector2d::new(*x as i32, y as i32);

            full_possibilities.iter().filter(|p| test_possibility(&input, &pos, p))
                .count() == 2
        }).count()
    }).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(""), Vec::<Vec<char>>::new());
        assert_eq!(parse_input("XMAS"), vec!(vec!('X', 'M', 'A', 'S')));
        assert_eq!(parse_input("XMAS\nXXXX"), vec!(vec!('X', 'M', 'A', 'S'),vec!('X', 'X', 'X', 'X')));
    }

    #[test]
    fn test_is_word(){
        let map_true = parse_input("XMAS");
        let map_false = parse_input("XMDAS");
        assert_eq!(count_word(&map_true, &Vector2d::new(0,0), "XMAS"), 1);
        assert_eq!(count_word(&map_false, &Vector2d::new(0,0), "XMAS"), 0);
        assert_eq!(count_word(&map_true, &Vector2d::new(1,0), "XMAS"), 0);
    }
}
