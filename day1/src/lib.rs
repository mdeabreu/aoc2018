use std::collections::HashSet;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Ok(Config {
                filename: String::from("input/day1.txt"),
            });
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let sum = part1(&contents);
    println!("Part 1 result: {}", sum);

    let sum = part2(&contents);
    println!("Part 2 result: {}", sum);

    Ok(())
}

fn part1(contents: &String) -> i32 {
    let mut sum: i32 = 0;

    for line in contents.lines() {
        let num: i32 = line.parse().unwrap();
        sum += num;
    }

    sum
}

fn part2(contents: &String) -> i32 {
    let mut sum: i32 = 0;
    let mut sums = HashSet::new();
    sums.insert(0);

    loop {
        for line in contents.lines() {
            let num: i32 = line.parse().unwrap();
            sum += num;
            if sums.contains(&sum) {
                return sum;
            } else {
                sums.insert(sum);
            }
        }
    }
}

#[cfg(test)]
mod part1 {
    use super::*;

    #[test]
    fn all_positive() {
        let contents = String::from(
            "\
+1
+1
+1
",
        );
        let sum = part1(&contents);
        assert_eq!(3, sum);
    }

    #[test]
    fn all_negative() {
        let contents = String::from(
            "\
-1
-2
-3
",
        );

        let sum = part1(&contents);
        assert_eq!(-6, sum);
    }

    #[test]
    fn zero() {
        let contents = String::from(
            "\
+1
+1
-2
",
        );

        let sum = part1(&contents);
        assert_eq!(0, sum);
    }
}

#[cfg(test)]
mod part2 {
    use super::*;

    #[test]
    fn zero() {
        let contents = String::from(
            "\
+1
-1
",
        );

        let sum = part2(&contents);
        assert_eq!(0, sum);
    }

    #[test]
    fn ten() {
        let contents = String::from(
            "\
+3
+3
+4
-2
-4
",
        );

        let sum = part2(&contents);
        assert_eq!(10, sum);
    }

    #[test]
    fn five() {
        let contents = String::from(
            "\
-6
+3
+8
+5
-6
",
        );

        let sum = part2(&contents);
        assert_eq!(5, sum);
    }

    #[test]
    fn fourteen() {
        let contents = String::from(
            "\
+7
+7
-2
-7
-4
",
        );

        let sum = part2(&contents);
        assert_eq!(14, sum);
    }
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    #[ignore]
    fn part1() {
        let config = Config {
            filename: String::from("../input/day1.txt"),
        };
        let contents = fs::read_to_string(config.filename).unwrap();
        let sum = super::part1(&contents);

        assert_eq!(442, sum);
    }

    #[test]
    #[ignore]
    fn part2() {
        let config = Config {
            filename: String::from("../input/day1.txt"),
        };
        let contents = fs::read_to_string(config.filename).unwrap();
        let sum = super::part2(&contents);

        assert_eq!(59908, sum);
    }
}
