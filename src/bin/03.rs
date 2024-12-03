use regex::Regex;

advent_of_code::solution!(3);

fn parse_input(str: &str) -> Vec<Vec<(u32, u32)>> {
    let regex = Regex::new(r##"mul\((\d+),(\d+)\)"##).unwrap();

    str.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            regex.captures_iter(line).map(|c| (
                c.get(1).unwrap().as_str().parse().unwrap(), c.get(2).unwrap().as_str().parse().unwrap()
            )).collect()
        }).collect()
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Compute {
    DO,
    DONT,
    MUL(u32, u32),
}

fn parse_input_2(str: &str) -> Vec<(Compute, u32)> {
    let regex = Regex::new(r##"(mul\((\d+),(\d+)\))"##).unwrap();
    let regex_do = Regex::new(r##"do\(\)"##).unwrap();
    let regex_dont = Regex::new(r##"don't\(\)"##).unwrap();

    let mut result = Vec::new();

    str.match_indices(&regex).into_iter().for_each(|(index, str)| {
        let capture = regex.captures(str).unwrap();
        result.push(
            (
                Compute::MUL(capture.get(2).unwrap().as_str().parse().unwrap(), 
                capture.get(3).unwrap().as_str().parse().unwrap()
            ), index as u32));
    });
    str.match_indices(&regex_do).into_iter().for_each(|(index, _)| {
        result.push((Compute::DO, index as u32));
    });
    str.match_indices(&regex_dont).into_iter().for_each(|(index, _)| {
        result.push((Compute::DONT, index as u32));
    });

    result.sort_by_key(|e| e.1);
    result
}

fn get_active_mul(input: Vec<(Compute, u32)>) -> Vec<(u32, u32)> {
    let mut active = true;

    input.iter().filter(|(state, _)| {
        match state {
            Compute::DO => active = true,
            Compute::DONT => active = false,
            _ => ()
        };
        active
    }).filter_map(|(e, _)| {
        if let Compute::MUL(a, b) = e {
            Some((*a, *b))
        } else {
            None
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let mul = input.iter().flatten().map(|(a, b)| a*b);
    Some(mul.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input_2(input);
    let active_mul = get_active_mul(input);
    let mul = active_mul.iter().map(|(a, b)| a*b);
    Some(mul.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(""), Vec::<Vec<_>>::new());
        assert_eq!(parse_input("fdsifjdgjdofgjidsjfoajdsiojgdf"), vec![vec![]]);
        assert_eq!(parse_input("mul(1,2)"), vec![vec![(1, 2)]]);
        assert_eq!(parse_input("mul(1,2)mul(2,3)"), vec![vec![(1, 2), (2, 3)]]);
        assert_eq!(parse_input("mul(1,2)\nmul(2,3)"), vec![vec![(1, 2)], vec![(2, 3)]]);
        assert_eq!(parse_input("mul(1,2)\ndsdfdsf"), vec![vec![(1, 2)], vec![]]);
    }
    #[test]
    fn test_parse_input_2() {
        use Compute::*;
        assert_eq!(parse_input_2(""), vec![]);
        assert_eq!(parse_input_2("fdsifjdgjdofgjidsjfoajdsiojgdf"), vec![]);
        assert_eq!(parse_input_2("mul(1,2)"), vec![(MUL(1, 2), 0)]);
        assert_eq!(parse_input_2("mul(1,2)mul(2,3)"), vec![(MUL(1,2), 0), (MUL(2,3), 8)]);
        assert_eq!(parse_input_2("don't()mul(1,2)"), vec![(DONT, 0), (MUL(1, 2), 7)]);
        assert_eq!(parse_input_2("do()don't()mul(1,2)"), vec![(DO, 0), (DONT, 4), (MUL(1, 2), 11)]);
    }

    #[test]
    fn test_get_active_mul() {
        use Compute::*;
        assert_eq!(get_active_mul(vec!()), vec!());
        assert_eq!(get_active_mul(vec!((MUL(1,2), 0))), vec!((1,2)));
        assert_eq!(get_active_mul(vec!((DO, 0),(MUL(1,2), 3))), vec!((1,2)));
        assert_eq!(get_active_mul(vec!((DONT, 0),(MUL(1,2), 3))), vec!());
        assert_eq!(get_active_mul(vec!((DONT, 0), (DO, 3),(MUL(1,2), 6))), vec!((1,2)));
    }
}
