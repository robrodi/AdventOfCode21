use std::path;

use pathfinding::directed::dijkstra;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let goal = (map[0].len() as i32 - 1, map.len() as i32 - 1);
    let path_density = dijkstra::dijkstra(
        &(0, 0),
        |(x, y)| {
            NEXT.iter()
                .map(|(xx, yy)| {
                    map.get((y + yy) as usize)
                        .and_then(|r| r.get((x + xx) as usize))
                        .map(|c| ((x + xx, y + yy), *c as u32))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == goal,
    )
    .unwrap()
    .1;
    Some(path_density)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> Vec<Vec<u8>>{
    input.lines()
        .map(|s| {
                s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
