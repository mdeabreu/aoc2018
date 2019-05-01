extern crate clap;

use clap::ArgMatches;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("Using input: {}", matches.value_of("INPUT").unwrap());

    let contents = fs::read_to_string(matches.value_of("INPUT").unwrap())?;

    let checksum = part1(&contents);
    println!("Checksum: {}", checksum);

    let common_letters = part2(&contents);
    println!("Common letters: {}", common_letters);

    Ok(())
}

fn part1(contents: &String) -> i32 {
    let mut pairs = 0;
    let mut trips = 0;
    for line in contents.lines() {
        if has_pairs(line) {
            pairs += 1;
        }
        if has_trips(line) {
            trips += 1;
        }
    }

    pairs * trips
}

fn part2<'a>(contents: &String) -> String {
    let lines: Vec<&str> = contents.lines().collect();
    for a in 0..lines.len() {
        for b in a+1..lines.len() {
            let num = num_differences(lines[a], lines[b]);
            if num == 1 {
                return find_common_letters(lines[a], lines[b]);
            }
        }
    }

    "".to_string()
}

fn find_common_letters<'a>(a: &str, b: &str) -> String {
    assert_eq!(a.len(), b.len());

    let mut common_letters = String::new();

    let iter = a.chars().zip(b.chars());
    for (i, j) in iter {
        if i == j {
            common_letters.push(i);
        }
    }

    common_letters
}

fn num_differences(a: &str, b: &str) -> i32 {
    assert_eq!(a.len(), b.len());

    let mut count = 0;
    let iter = a.chars().zip(b.chars());
    for (i, j) in iter {
        if i != j {
            count += 1;
        }
    }

    count
}

fn has_pairs(id: &str) -> bool {
    let letters = count_entries(id);

    for val in letters.values() {
        match val {
            2 => return true,
            _ => continue,
        }
    }
    false
}

fn has_trips(id: &str) -> bool {
    let letters = count_entries(id);

    for val in letters.values() {
        match val {
            3 => return true,
            _ => continue,
        }
    }
    false
}

fn count_entries(id: &str) -> HashMap<char, i32> {
    let mut letters: HashMap<char, i32> = HashMap::new();
    for letter in id.chars() {
        let counter = letters.entry(letter).or_insert(0);
        *counter += 1;
    }

    letters
}

#[cfg(test)]
mod abcdef {
    use super::*;
    const ID: &str = "abcdef";
    #[test]
    fn no_pairs() {
        assert!(!has_pairs(ID));
    }
    #[test]
    fn no_trips() {
        assert!(!has_trips(ID));
    }
}

#[cfg(test)]
mod bababc {
    use super::*;
    const ID: &str = "bababc";
    #[test]
    fn yes_pairs() {
        assert!(has_pairs(ID));
    }
    #[test]
    fn yes_trips() {
        assert!(has_trips(ID));
    }
}

#[cfg(test)]
mod abbcde {
    use super::*;
    const ID: &str = "abbcde";
    #[test]
    fn yes_pairs() {
        assert!(has_pairs(ID));
    }
    #[test]
    fn no_trips() {
        assert!(!has_trips(ID));
    }
}

#[cfg(test)]
mod abcccd {
    use super::*;
    const ID: &str = "abcccd";
    #[test]
    fn no_pairs() {
        assert!(!has_pairs(ID));
    }
    #[test]
    fn yes_trips() {
        assert!(has_trips(ID));
    }
}

#[cfg(test)]
mod aabcdd {
    use super::*;
    const ID: &str = "aabcdd";
    #[test]
    fn yes_pairs() {
        assert!(has_pairs(ID));
    }
    #[test]
    fn no_trips() {
        assert!(!has_trips(ID));
    }
}

#[cfg(test)]
mod abcdee {
    use super::*;
    const ID: &str = "abcdee";
    #[test]
    fn yes_pairs() {
        assert!(has_pairs(ID));
    }
    #[test]
    fn no_trips() {
        assert!(!has_trips(ID));
    }
}

#[cfg(test)]
mod ababab {
    use super::*;
    const ID: &str = "ababab";
    #[test]
    fn no_pairs() {
        assert!(!has_pairs(ID));
    }
    #[test]
    fn yes_trips() {
        assert!(has_trips(ID));
    }
}

#[cfg(test)]
mod num_differences {
    use super::*;

    #[test]
    fn multiple_differences() {
        let a = "abcde";
        let b = "axcye";

        let num = num_differences(a, b);
        assert_eq!(2, num);
    }

    #[test]
    fn one_difference() {
        let a = "fghij";
        let b = "fguij";

        let num = num_differences(a, b);
        assert_eq!(1, num);
    }
}

#[cfg(test)]
mod samples {
    #[test]
    fn part1() {
            let ids: &str = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";
        let checksum = super::part1(&String::from(ids));
        assert_eq!(12, checksum);
    }

    #[test]
    fn part2() {
        let ids: &str = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";
        let ids = String::from(ids);
        let common_letters = super::part2(&String::from(ids));
        assert_eq!("fgij", common_letters);
    }
}
