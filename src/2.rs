use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
use Hand::*;

enum Outcome {
    Win,
    Loss,
    Draw,
}
use Outcome::*;

impl Outcome {
    fn parse(letter: char) -> Outcome {
        match letter {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("invalid character {}", letter),
        }
    }

    fn determine_hand(&self, them: Hand) -> Hand {
        match (them, self) {
            (Rock, Win) | (Paper, Draw) | (Scissors, Loss) => Paper,
            (Paper, Win) | (Scissors, Draw) | (Rock, Loss) => Scissors,
            (Scissors, Win) | (Rock, Draw) | (Paper, Loss) => Rock,
        }
    }
}

impl Hand {
    fn score(&self, them: Hand) -> i64 {
        let hand_score = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let round_score = match (self, them) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => 3,
            _ => 0,
        };
        hand_score + round_score
    }

    fn parse(letter: char) -> Hand {
        match letter {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("invalid character {}", letter),
        }
    }
}

fn pt1(input: &str) -> i64 {
    input.lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let them = Hand::parse(chars[0]);
            let us = Hand::parse(chars[2]);
            us.score(them)
        })
        .sum()
}

fn pt2(input: &str) -> i64 {
    input.lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let them = Hand::parse(chars[0]);
            let outcome = Outcome::parse(chars[2]);
            let us = outcome.determine_hand(them);
            us.score(them)
        })
        .sum()
}

fn main() {
    dbg!(pt1(&read_to_string("data/2.txt").unwrap()));
    dbg!(pt2(&read_to_string("data/2.txt").unwrap()));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test1() {
        assert_eq!(pt1(TEST_INPUT), 15);
    }

    #[test]
    fn test2() {
        assert_eq!(pt2(TEST_INPUT), 12);
    }
}