use std::collections::HashMap;

advent_of_code::solution!(11);

fn count_blink(n: usize, turns: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(res) = cache.get(&(n, turns)) {
        return *res;
    }
    
    let res = if turns == 0 {
        1
    } else if n == 0 {
        count_blink(1, turns-1, cache)
    } else {
        let str = n.to_string();
        if str.len() % 2 == 0 {
            let left = &str[0..str.len()/2].parse::<usize>().unwrap();
            let right = &str[str.len()/2..str.len()].parse::<usize>().unwrap();
    
            count_blink(*left, turns-1, cache) + count_blink(*right, turns-1, cache)
        } else {
            count_blink(n*2024, turns-1, cache)
        }
    };

    cache.insert((n, turns), res);
    
    res
}

type Input = Vec<usize>;
fn parse(input: &str) -> Input {
    input.trim_matches('\n').split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut cache = HashMap::new();
    let sum = input.into_iter().map(|n| count_blink(n, 25, &mut cache)).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut cache = HashMap::new();
    let sum = input.into_iter().map(|n| count_blink(n, 75, &mut cache)).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    }

    #[test]
    fn test_count_blink(){
        assert_eq!(count_blink(125, 1, &mut HashMap::new()), 1);
        assert_eq!(count_blink(125, 2, &mut HashMap::new()), 2);
    }
}
