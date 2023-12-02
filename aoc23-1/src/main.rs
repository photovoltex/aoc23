use std::fmt::Display;

// https://adventofcode.com/2023/day/1
const EXAMPLE_1: (&str, &str) = ("example_1", include_str!("../example_1.txt"));
const EXAMPLE_2: (&str, &str) = ("example_2", include_str!("../example_2.txt"));
const INPUT: (&str, &str) = ("input", include_str!("../input.txt"));

fn main() {
    for (name, input) in [EXAMPLE_1, INPUT] {
        evaluate(name, part_one(input));
    }

    for (name, input) in [EXAMPLE_2, INPUT] {
        evaluate(name, part_two(input));
    }
}

fn evaluate<T>(name: &str, numbers: Vec<(T, T)>)
where
    T: Display,
{
    let size = numbers.len();

    let sum = numbers
        .into_iter()
        .flat_map(|(first, last)| format!("{first}{last}").parse::<u32>())
        .sum::<u32>();

    println!("[{name}] with [{size}] values and sum of [{sum}]");
}

fn part_one(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let numbers = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            (*numbers.first().unwrap(), *numbers.last().unwrap())
        })
        .collect()
}

const PATTERNS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn to_num(num: &str) -> u8 {
    match num {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        numeric => numeric.parse().unwrap(),
    }
}

fn part_two(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|line| {
            let mut matches = PATTERNS
                .into_iter()
                .flat_map(|pattern| {
                    vec![
                        line.find(pattern).map(|i| (i, &line[i..i + pattern.len()])),
                        line.rfind(pattern)
                            .map(|i| (i, &line[i..i + pattern.len()])),
                    ]
                })
                .flatten()
                .collect::<Vec<_>>();

            matches.sort_by(|(i1, _), (i2, _)| i1.partial_cmp(i2).unwrap());

            let (_, first) = matches.first().unwrap();
            let (_, last) = matches.last().unwrap();

            (to_num(first), to_num(last))
        })
        .collect()
}

#[cfg(test)]
mod evil_test {
    #[test]
    fn melted_together() {
        assert_eq!(vec![(8, 3)], super::part_two("eighthree"))
    }

    #[test]
    fn duplicate() {
        assert_eq!(vec![(4, 2)], super::part_two("bxfour3two2sb4twondmfdpsz"))
    }
}
