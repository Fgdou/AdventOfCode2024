use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

struct Input {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Input {
    let mut splitted = input.split("\n\n");
    let rules = splitted.next().unwrap()
        .split("\n")
        .map(|line| line.split("|").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>())
        .map(|list| (*list.get(0).unwrap(), *list.get(1).unwrap()))
        .collect::<Vec<_>>();
    let updates = splitted.next().unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    Input{
        rules,
        updates,
    }
}

fn transform_rules(rules: &Vec<(usize, usize)>) -> HashMap<usize, HashSet<usize>> {
    let mut map : HashMap<usize, HashSet<usize>> = HashMap::new();

    for rule in rules {
        map.entry(rule.0).or_insert(HashSet::new()).insert(rule.1);
    }

    map
}

fn check_rule_and_put_number_back(numbers: &mut Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) {
    let len = numbers.len();
    let mut i = 0;
    while i != len {
        let n1 = *numbers.get(i).unwrap();
        let ordered = (0..i).all(|i| {
            let n2 = numbers.get(i).unwrap();
            match rules.get(&n1) {
                None => true,
                Some(set) => !set.contains(n2)
            }
        });

        if !ordered {
            *numbers.get_mut(i).unwrap() = *numbers.get(i-1).unwrap();
            *numbers.get_mut(i-1).unwrap() = n1;
            i -= 1;
        } else {
            i += 1;
        }
    }
}

fn check_rules_for_numbers(numbers: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    numbers.iter().enumerate().all(|(index, n1)| {
        (0..index).all(|i| {
            let n2 = numbers.get(i).unwrap();
            match rules.get(n1) {
                None => true,
                Some(set) => !set.contains(n2)
            }
        })
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse_input(input);
    let rules = transform_rules(&input.rules);

    let sum = input.updates.iter()
        .filter(|update| check_rules_for_numbers(&update, &rules))
        .map(|numbers| numbers.get(numbers.len()/2).unwrap())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse_input(input);
    let rules = transform_rules(&input.rules);

    let sum = input.updates.clone()
        .iter_mut()
        .filter(|update| !check_rules_for_numbers(&update, &rules))
        .map(|update| {
            check_rule_and_put_number_back(update, &rules);
            update
        })
        .map(|numbers| numbers.get(numbers.len()/2).unwrap())
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
