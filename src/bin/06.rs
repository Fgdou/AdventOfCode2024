use std::collections::HashSet;

use vectorlib::math::vector2d_module::Vector2d;

advent_of_code::solution!(6);

#[derive(Debug)]
enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl Direction {
    fn next_90_angle(&self) -> Direction {
        use Direction::*;
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: Vector2d<i32>,
    direction: Direction,
}

struct Input {
    guard_start: Vector2d<i32>,
    obstacles: Obstacles,
    map_size: Vector2d<i32>,
}

type Obstacles = HashSet<(i32, i32)>;

impl Guard {
    fn next_pos(&self) -> Vector2d<i32> {
        let delta = match self.direction {
            Direction::UP => Vector2d::new(0, -1),
            Direction::RIGHT => Vector2d::new(1, 0),
            Direction::DOWN => Vector2d::new(0, 1),
            Direction::LEFT => Vector2d::new(-1, 0),
        };

        self.position + delta
    }
    fn walk(&mut self, obstacles: &Obstacles) {
        for _ in 0..4 {
            let next_pos = self.next_pos();
            if !obstacles.contains(&(next_pos.x, next_pos.y)) {
                self.position = next_pos;
                return;
            } else {
                self.direction = self.direction.next_90_angle();
            }
        }
        panic!()
    }
}

fn is_inside(map_size: Vector2d<i32>, pos: Vector2d<i32>) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < map_size.x && pos.y < map_size.y
}

fn parse(input: &str) -> Input {
    let mut obstacles = HashSet::new();
    let mut guard_position = None;

    let lines = input.split("\n")
        .filter(|line| !line.is_empty()).collect::<Vec<_>>();

    let height = lines.len();
    let mut width = None;

    for (y, line) in lines.iter().enumerate() {
        width = Some(line.len());
        for (x, c) in line.char_indices() {
            match c {
                '^' => guard_position = Some(Vector2d::new(x as i32, y as i32)),
                '#' => {obstacles.insert((x as i32, y as i32));},
                _ => ()
            }
        }
    }


    Input{
        guard_start: guard_position.unwrap(),
        obstacles,
        map_size: Vector2d::new(width.unwrap() as i32, height as i32),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);


    let mut guard = Guard{
        position: input.guard_start,
        direction: Direction::UP,
    };

    let mut history = HashSet::new();

    while is_inside(input.map_size, guard.position) {
        history.insert((guard.position.x, guard.position.y));
        guard.walk(&input.obstacles);
    }

    Some(history.len())
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
