advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| 
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        ).collect()
}

fn derivative(list: &Vec<i32>) -> Vec<i32> {
    assert!(list.len() > 1);
    (0..list.len()-1).into_iter().map(|i| 
        list[i+1] - list[i]
    ).collect()
}

fn same_sign(list: &Vec<i32>) -> bool {
    assert!(list.len() > 0);

    let mut sign = None;

    for n in list {
        sign = match (sign, n.signum()) {
            (_, 0) => sign,
            (None, s) => Some(s),
            (Some(s1), s2) => {
                if s1 == s2 {
                    sign
                } else {
                    return false
                }
            }
        }
    }

    true
}
fn all_between(list: &Vec<i32>, a: i32, b: i32) -> bool {
    list.iter().all(|n| *n >= a && *n <= b)
}

pub fn part_one(input: &str) -> Option<i32> {
    let list = parse_input(input);

    let safe: Vec<bool> = list.iter().map(|numbers| {
        // derivative
        let derivatives = derivative(numbers);
        let derivatives_abs: Vec<i32> = derivatives.iter().map(|n| n.abs()).collect();
        // verify same sign
        // verify between 1 and 3
        same_sign(&derivatives) && all_between(&derivatives_abs, 1, 3)
    }).collect();

    // sum all results
    Some(safe.iter().filter(|i| **i).count() as i32)
}

fn generate_variations(list: &Vec<i32>) -> Vec<Vec<i32>> {
    (0..list.len()).into_iter().map(|i| {
        let mut list = list.clone();
        list.remove(i);
        list
    }).collect()
}

pub fn part_two(input: &str) -> Option<i32> {
    let list = parse_input(input);

    let safe: Vec<bool> = list.iter().map(|numbers| {
        let variations = generate_variations(numbers);

        variations.iter().any(|numbers| {
            // derivative
            let derivatives = derivative(numbers);
            let derivatives_abs: Vec<i32> = derivatives.iter().map(|n| n.abs()).collect();
            // verify same sign
            // verify between 1 and 3
            same_sign(&derivatives) && all_between(&derivatives_abs, 1, 3)
        })
    }).collect();

    // sum all results
    Some(safe.iter().filter(|i| **i).count() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_derivative() {
        assert_eq!(derivative(&vec!(1, 2, 3, 4)), vec!(1, 1, 1));
    }

    #[test]
    fn test_same_sign() {
        assert_eq!(same_sign(&vec!(1, 2, 3, 0, 4)), true);
        assert_eq!(same_sign(&vec!(1, 2, -3, 0, 4)), false);
        assert_eq!(same_sign(&vec!(1, 2, -3, -0, 4)), false);
        assert_eq!(same_sign(&vec!(1, 2, 3, -0, 4)), true);
        assert_eq!(same_sign(&vec!(-1, 2, 3, 0, -4)), false);
        assert_eq!(same_sign(&vec!(0, 2, 3, 0, -4)), false);
        assert_eq!(same_sign(&vec!(0, 2, 3, 0, 4)), true);
        assert_eq!(same_sign(&vec!(0, -2, 3, 0, 4)), false);
    }

    #[test]
    fn test_all_between() {
        assert_eq!(all_between(&vec!(), 1, 3), true);
        assert_eq!(all_between(&vec!(1, 1, 3, 2, 1), 1, 3), true);
        assert_eq!(all_between(&vec!(1, 1, 3, 2, 0), 1, 3), false);
    }
}
