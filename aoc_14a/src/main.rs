use regex::Regex;
use std::collections::HashMap;

fn prepare_aoc_14a_input(filename: &str) -> Result<(Vec<u8>, HashMap<[u8; 2], u8>), std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let mut lines = content.lines();

    let sequence = lines.next().unwrap().bytes().collect();
    lines.next();

    let line_regex = Regex::new(r"(..) -> (.)").unwrap();
    let mut rules: HashMap<[u8; 2], u8> = HashMap::new();
    for line in lines.filter(|line| !line.is_empty()) {
        let captures = line_regex.captures(line).unwrap();
        let pair: [u8; 2] = captures.get(1).unwrap().as_str().as_bytes().try_into().unwrap();
        let element = captures.get(2).unwrap().as_str().as_bytes()[0];
        rules.insert(pair, element);
    }

    Ok((sequence, rules))
}

fn solve_aoc_14a(sequence: &[u8], rules: &HashMap<[u8; 2], u8>) -> usize {
    //println!("{:?}\n{:?}", sequence, rules);

    let mut current_sequence = sequence.to_vec();

    for _ in 0..10 {
        let mut new_sequence: Vec<u8> = Vec::new();
        new_sequence.push(current_sequence[0]);

        for i in 0..current_sequence.len() - 1 {
            let pair: &[u8; 2] = &current_sequence[i..i+2].try_into().unwrap();
            if rules.contains_key(pair) {
                new_sequence.push(rules[pair]);
            }
            new_sequence.push(pair[1]);
        }

        current_sequence = new_sequence;
    }

    //println!("{:?}", current_sequence);

    let mut histogram: [usize; 128] = [0; 128];

    for value in current_sequence {
        histogram[value as usize] += 1;
    }

    //println!("{:?}", histogram);

    let mut min: (u8, usize) = (0, usize::MAX);
    let mut max: (u8, usize) = (0, 0);

    for (i, value) in histogram.into_iter().enumerate() {
        max = if value > max.1 { (i as u8, value) } else { max };
        min = if value != 0 && value < min.1 { (i as u8, value) } else { min };
    }

    //println!("{:?} {:?}", min, max);

    max.1 - min.1
}

fn main() -> Result<(), std::io::Error> {
    let (sequence, rules) = prepare_aoc_14a_input("input/input.txt")?;
    let result = solve_aoc_14a(&sequence, &rules);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_14a() {
        let (sequence, rules) = prepare_aoc_14a_input("input/test.txt").unwrap();
        let result = solve_aoc_14a(&sequence, &rules);

        assert_eq!(result, 1588)
    }
}
