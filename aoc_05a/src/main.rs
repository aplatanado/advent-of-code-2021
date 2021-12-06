use regex::Regex;
use std::str::FromStr;

type Coord = u16;
type Point = (Coord ,Coord);
const GRID_SIZES: (usize, usize) = (1024,1024);

fn prepare_aoc_05a_input(filename: &str) -> Result<Vec<(Point,Point)>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;

    let line_regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    Ok(content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let captures = line_regex.captures(line).unwrap();
            let x0 = Coord::from_str(captures.get(1).unwrap().as_str()).unwrap();
            let y0 = Coord::from_str(captures.get(2).unwrap().as_str()).unwrap();
            let x1 = Coord::from_str(captures.get(3).unwrap().as_str()).unwrap();
            let y1 = Coord::from_str(captures.get(4).unwrap().as_str()).unwrap();
            ((x0, y0), (x1, y1))
        }).collect())
}

fn raycast(line: &(Point,Point)) -> Option<Vec<Point>> {
    if line.0.0 == line.1.0 {
        let (start, end) = if line.0.1 <= line.1.1 { (line.0.1, line.1.1) } else { (line.1.1, line.0.1) };
        Some(std::ops::RangeInclusive::new(start, end).map(|y| (line.0.0, y)).collect())
    }
    else if line.0.1 == line.1.1 {
        let (start, end) = if line.0.0 <= line.1.0 { (line.0.0, line.1.0) } else { (line.1.0, line.0.0) };
        Some(std::ops::RangeInclusive::new(start, end).map(|x| (x, line.0.1)).collect())
    }
    else {
        None
    }
}

fn search_overlap(grid: &[[u8; GRID_SIZES.0]; GRID_SIZES.0]) -> usize {
    let mut count = 0;
    for line in grid {
        count += line.iter().filter(|value| **value > 1).count();
    }

    count
}

fn solve_aoc_05a(input: &[(Point,Point)]) -> usize {
    //println!("{:?}", input);

    let mut grid: [[u8; GRID_SIZES.0]; GRID_SIZES.1] = [[0; GRID_SIZES.0]; GRID_SIZES.1];

    for line in input {
        let trace = raycast(line);
        //println!("{:?}", trace);

        if trace.is_none() {
            continue
        }

        for point in trace.unwrap() {
            grid[point.1 as usize][point.0 as usize] += 1;
        }
    }

    //println!("{:?}", grid);

    search_overlap(&grid)
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_05a_input("input/input.txt")?;
    let result = solve_aoc_05a(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_02a() {
        let input = prepare_aoc_05a_input("input/test.txt").unwrap();
        let result = solve_aoc_05a(&input);

        assert_eq!(result, 5)
    }
}