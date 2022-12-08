use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    let sizes = input.lines().skip(2)
        .fold((Vec::new(), HashMap::new()), |(mut cd, mut dirs), line| {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("$") => match (parts.next(), parts.next()) {
                    (Some("cd"), Some("..")) => { cd.pop(); },
                    (Some("cd"), Some(dir)) => cd.push(dir),
                    _ => {},
                },
                Some(size) => *dirs.entry(format!("/{}", cd.join("/"))).or_insert(0) += size.parse().unwrap_or(0),
                _ => {},
            }
            (cd, dirs)
        }).1;
    sizes.keys()
        .map(|dir| sizes.iter()
            .filter(|(path, _)| path.starts_with(dir))
            .map(|(_, size)| size)
            .sum())
        .collect()
}

fn pt1(input: &str) -> usize {
    parse(input).iter().filter(|size| **size <= 100000).sum()
}

fn pt2(input: &str) -> usize {
    let dirs = parse(input);
    let needed = 30000000 - (70000000 - dirs.iter().max().unwrap());
    dirs.iter().fold(usize::MAX, |min, size| if *size >= needed && *size < min { *size } else { min })
}

fn main () {
    dbg!(pt1(&std::fs::read_to_string("data/7.txt").unwrap()));
    dbg!(pt2(&std::fs::read_to_string("data/7.txt").unwrap()));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test1() {
        assert_eq!(pt1(TEST_INPUT), 95437)
    }

    #[test]
    fn test2() {
        assert_eq!(pt2(TEST_INPUT), 24933642)
    }
}