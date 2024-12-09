
advent_of_code::solution!(9);

type Drive = Vec<Part>;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Part {
    count: usize,
    id: Option<usize>,
}


fn parse(input: &str) -> Drive {
    input.trim_matches('\n').chars().enumerate()
    .map(|(i, n)| {
        let count = n.to_string().parse::<usize>().unwrap();
        let id = match i%2 == 0 {
            false => None, 
            true => Some(i/2)
        };
        Part{
            count, 
            id
        }
    })
    .collect()
}

fn walk_and_fill(mut drive: Drive) -> Drive {
    let mut left = 0;
    let mut right = drive.len()-1;

    while left <= right {
        let left_value = drive.get(left).unwrap().clone();
        let right_value = drive.get(right).unwrap().clone();

        match (left_value.id, right_value.id) {
            (_, Some(0)) => {
                drive.pop();
                right -= 1;
            },
            (None, None) => {
                drive.pop();
                right -= 1;
            },
            (Some(_), _) => {
                left += 1;
            },
            (None, Some(_)) => {
                if left_value.count <= right_value.count {
                    drive.get_mut(right).unwrap().count -= left_value.count;
                    drive.get_mut(left).unwrap().id = right_value.id;
                } else {
                    drive.get_mut(left).unwrap().count -= right_value.count;
                    drive.pop();
                    drive.insert(left, right_value);
                }
            }
        }
    }

    drive
}

fn calculate_checksum(drive: &Drive) -> usize {
    let mut i = 0;
    drive.iter().filter_map(|part| {
        let id = part.id?;
        let sum = (0..part.count).map(|_| {
            let calc = i*id;
            i += 1;
            calc
        }).sum::<usize>();
        Some(sum)
    }).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let filled = walk_and_fill(input);
    Some(calculate_checksum(&filled))
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("12345"), vec![
            Part{count:1, id:Some(0)},
            Part{count:2, id:None},
            Part{count:3, id:Some(1)},
            Part{count:4, id:None},
            Part{count:5, id:Some(2)},
        ])
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_walk() {
        let drive = vec!(
            Part{count:1, id:Some(0)},
            Part{count:2, id:None},
            Part{count:3, id:Some(1)},
            Part{count:4, id:None},
            Part{count:5, id:Some(2)},
        );
        let expected_drive = vec!(
            Part{count:1, id:Some(0)},
            Part{count:2, id:Some(2)},
            Part{count:3, id:Some(1)},
            Part{count:3, id:Some(2)},
        );
        let drive = walk_and_fill(drive);
        assert_eq!(drive, expected_drive);
    }
}
