fn parse_elves(input: &str) -> impl Iterator<Item = ((i64, i64), (i64, i64))> + '_ {
    input.lines()
        .map(|line| line.split(',')
            .flat_map(|ranges| ranges.split('-'))
            .flat_map(|num| num.parse::<i64>())
            .collect::<Vec<i64>>())
        .map(|nums| ((nums[0], nums[1]), (nums[2], nums[3])))
}

fn pt1(input: &str) -> usize {
    parse_elves(input)
        .filter(|(elf1, elf2)| {
            if elf1.0 == elf2.0 {
                return true;
            }
            if elf1.0 < elf2.0 {
                elf2.1 <= elf1.1
            } else {
                elf1.1 <= elf2.1
            }
        })
        .count()
}

fn pt2(input: &str) -> usize {
    parse_elves(input)
        .filter(|(elf1, elf2)| {
            if elf1.0 == elf2.0 {
                return true;
            }
            if elf1.0 < elf2.0 {
                elf1.1 >= elf2.0
            } else {
                elf2.1 >= elf1.0
            }
        })
        .count()
}

fn main() {
    dbg!(pt1(&std::fs::read_to_string("data/4.txt").unwrap()));
    dbg!(pt2(&std::fs::read_to_string("data/4.txt").unwrap()));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test1() {
        assert_eq!(pt1(TEST_INPUT), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(pt2(TEST_INPUT), 4);
    }
}