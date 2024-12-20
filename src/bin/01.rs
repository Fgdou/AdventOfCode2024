advent_of_code::solution!(1);

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let numbers: Vec<Vec<i32>> = input.split("\n")
    .filter(|line| !line.is_empty())
    .map(|line| line.split_whitespace()
        .map(|n| n.parse().unwrap()).collect()
    ).collect();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for list in numbers {
        list1.push(list[0]);
        list2.push(list[1]);
    }

    (list1, list2)
}

pub fn part_one(input: &str) -> Option<i32> {
    // transformer en 2 listes
    let (mut list1, mut list2) = parse_input(input);
    

    // trie les listes
    list1.sort();
    list2.sort();

    // pour chaque élément, faire la différence
    let list_result = (0..list1.len()).into_iter().map(|line_number| {
        (list1[line_number] - list2[line_number]).abs()
    });

    // ajouter le résultat
    Some(list_result.sum())
}

pub fn count_iteration(list: &Vec<i32>, number: i32) -> i32 {
    list.iter().filter(|other_number| number == **other_number).count() as i32
}

pub fn part_two(input: &str) -> Option<i32> {
    // on veut nos 2 listes
    let (list1, list2) = parse_input(input);
    // pour chaque nombre de gauche, compter à droite et multiplier
    let result_list = list1.iter().map(|number| 
        number * count_iteration(&list2, *number)
    );

    Some(result_list.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
