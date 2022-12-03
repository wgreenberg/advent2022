use std::fs::read_to_string;

fn parse_elves(input: &str) -> Vec<i64> {
    input.split("\n\n")
        .map(|elf| elf.split("\n"))
        .map(|elf| elf.flat_map(|calories| calories.parse::<i64>()))
        .map(|elf| elf.sum())
        .collect()
}

fn pt1(input: &str) -> i64 {
    *parse_elves(input).iter().max().unwrap()
}

fn pt2(input: &str) -> i64 {
    let mut elves = parse_elves(input);
    elves.sort();
    elves.iter().rev().take(3).sum()
}

fn main() {
    dbg!(pt1(&read_to_string("data/1.txt").unwrap()));
    dbg!(pt2(&read_to_string("data/1.txt").unwrap()));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test1() {
        assert_eq!(pt1(TEST_INPUT), 24000)
    }

    #[test]
    fn test2() {
        assert_eq!(pt2(TEST_INPUT), 45000)
    }
}