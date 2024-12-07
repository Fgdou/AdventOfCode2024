use core::net;
use std::rc::Rc;

advent_of_code::solution!(7);

type Input = Vec<EquationInput>;

struct EquationInput {
    test_value: usize,
    numbers: Vec<usize>,
}

fn parse(input: &str) -> Input {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut equation = line.split(": ");

            let test_value = equation.next().unwrap().parse().unwrap();
            let numbers = equation.next().unwrap().split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            EquationInput{
                test_value,
                numbers
            }
        }).collect()
}

enum Operator {
    ADD,
    MULTIPLY,
    NO_OP,
}

fn calc(equation: &EquationInput, next_operator: &Operator, index_numbers: usize, computed_value: usize) -> bool {
    let next_number = match equation.numbers.get(index_numbers) {
        Some(n) => *n,
        None => {return equation.test_value == computed_value;},
    };

    let computed_value = match next_operator {
        Operator::ADD => computed_value + next_number,
        Operator::MULTIPLY => computed_value * next_number,
        Operator::NO_OP => next_number,
    };

    calc(equation, &Operator::ADD, index_numbers+1, computed_value) ||
        calc(equation, &Operator::MULTIPLY, index_numbers+1, computed_value)
}
fn calc_all(equation: &EquationInput) -> bool {
    calc(equation, &Operator::NO_OP, 0, 0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let sum = input.iter().filter(|equation| {
        calc_all(equation)
    }).map(|equation| equation.test_value)
    .sum();

    Some(sum)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
