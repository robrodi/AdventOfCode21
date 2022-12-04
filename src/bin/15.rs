use pathfinding::directed::dijkstra;
type Map = Vec<Vec<u8>>;
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
    let map = parse(input);
    let map_5h = ugh_5x_horiz(map);
    None
}
fn ugh_5x_horiz(map: Map) -> Map{
    let mut result: Map = Vec::new();
    for row in map{

        let mut new_row: Vec<u8> = Vec::new();
        for i in 0..5 {
            for j in &row {
                let mut value = j + i;
                if (value > 9) { value = (value % 10) + 1; }
                new_row.push(value);
            }
        }
        result.push(new_row);
    }
    result
}

fn parse(input: &str) -> Map{
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
    fn test_5xh(){
        let expected = "11637517422274862853338597396444961841755517295286";
        let input = parse(&advent_of_code::read_file("examples", 15));
        let actual: Vec<u8> = ugh_5x_horiz(input).first().unwrap().to_vec();
        let ss = actual.iter().map(|u| u.to_string()).collect::<Vec<String>>().join("");
        assert_eq!(ss, expected);
    }
    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(315));
    }
}
