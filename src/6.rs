fn first_run(input: &str, length: usize) -> usize {
    input.as_bytes().windows(length).enumerate()
        .filter(|(_, buf)| buf.iter().all(|c| buf.iter().filter(|x| *x == c).count() == 1))
        .map(|(i, _)| i + length)
        .next().unwrap()
}

fn pt1(input: &str) -> usize {
    first_run(input, 4)
}

fn pt2(input: &str) -> usize {
    first_run(input, 14)
}

fn main() {
    dbg!(pt1(&std::fs::read_to_string("data/6.txt").unwrap()));
    dbg!(pt2(&std::fs::read_to_string("data/6.txt").unwrap()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(pt1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(pt1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(pt1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(pt2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(pt2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(pt2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }
}