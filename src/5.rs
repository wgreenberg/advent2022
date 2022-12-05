use core::num;
use std::str::FromStr;

#[derive(Debug)]
struct Supplies {
    stacks: Vec<Vec<char>>,
}

impl Supplies {
    fn new() -> Supplies {
        Supplies { stacks: vec![Vec::new(); 9] }
    }

    fn handle_command(&mut self, command: &str, multiple: bool) {
        let parts: Vec<usize> = command.split(' ')
            .flat_map(|part| part.parse::<usize>())
            .collect();
        let (n, from, to) = (parts[0], parts[1], parts[2]);
        if multiple {
            let start = self.stacks[from - 1].len() - n;
            let chunk = self.stacks[from - 1].split_off(start);
            self.stacks[to - 1].extend(&chunk);
        } else {
            for _ in 0..n {
                let x = self.stacks[from - 1].pop().unwrap();
                self.stacks[to - 1].push(x);
            }
        }
    }
}

impl FromStr for Supplies {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut supplies = Supplies::new();
        s.lines()
            .take_while(|line| !line.starts_with(" 1"))
            .for_each(|line| {
                line.as_bytes()
                    .chunks(4)
                    .enumerate()
                    .for_each(|(i, letters)| {
                        let letter = letters[1] as char;
                        if letter != ' ' {
                            supplies.stacks[i].push(letter);
                        }
                    });
            });
        for stack in &mut supplies.stacks {
            stack.reverse();
        }
        Ok(supplies)
    }
}

fn run(input: &str, grab_multiple: bool) -> String {
    let mut supplies: Supplies = input.parse().unwrap();
    input.lines()
        .filter(|line| line.starts_with("move"))
        .for_each(|line| supplies.handle_command(line, grab_multiple));
    let mut result = String::new();
    for stack in &mut supplies.stacks {
        if let Some(c) = stack.pop() {
            result.push(c);
        }
    }
    result
}

fn pt1(input: &str) -> String {
    run(input, false)
}

fn pt2(input: &str) -> String {
    run(input, true)
}

fn main () {
    dbg!(pt1(&std::fs::read_to_string("data/5.txt").unwrap()));
    dbg!(pt2(&std::fs::read_to_string("data/5.txt").unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test1() {
        assert_eq!(pt1(TEST_INPUT), "CMZ".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(pt2(TEST_INPUT), "MCD".to_string());
    }
}