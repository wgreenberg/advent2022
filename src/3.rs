use std::fs::read_to_string;
use std::collections::HashSet;

fn common_char(strings: &[&str]) -> char {
    *strings.iter()
        .map(|string| HashSet::<char>::from_iter(string.chars()))
        .reduce(|acc, set| &acc & &set).unwrap()
        .iter()
        .next().unwrap()
}

fn score(letter: char) -> i64 {
    (match letter {
        'a'..='z' => (letter as u8) - 96,
        'A'..='Z' => (letter as u8) - 64 + 26,
        _ => panic!("nope"),
    }) as i64
}

fn pt1(input: &str) -> i64 {
    input.lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            score(common_char(&[left, right]))
        })
        .sum()
}

fn pt2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    (0..lines.len()).step_by(3)
        .map(|i| score(common_char(&lines[i..i+3])))
        .sum()
}

fn main() {
    dbg!(pt1(&read_to_string("data/3.txt").unwrap()));
    dbg!(pt2(&read_to_string("data/3.txt").unwrap()));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test1() {
        assert_eq!(pt1(TEST_INPUT), 157);
    }

    #[test]
    fn test2() {
        assert_eq!(pt2(TEST_INPUT), 70);
    }
}