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

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    let mul = input.iter().flatten().map(|(a, b)| a*b);
    Some(mul.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
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
}
