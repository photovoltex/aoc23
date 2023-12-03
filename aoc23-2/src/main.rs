use std::{fmt::Debug, iter::Sum, ops::Mul, str::FromStr};

// https://adventofcode.com/2023/day/2
const INPUT: &str = include_str!("../input.txt");

const ID_IDENTIFIER: &str = "Game ";

const RED_MAX: u16 = 12;
const GREEN_MAX: u16 = 13;
const BLUE_MAX: u16 = 14;

fn evaluate<S>(input: Vec<S>) -> S
where
    S: Sum,
{
    input.into_iter().sum()
}

fn main() {
    println!("part one: {}", evaluate(part_one(INPUT)));
    println!("part two: {}", evaluate(part_two::<u32>(INPUT)));
}

fn part_one(input: &str) -> Vec<u16> {
    input
        .lines()
        .flat_map(|line| {
            let start_str = &line.split(':').next().unwrap();

            let id = &start_str[ID_IDENTIFIER.len()..];
            let remaining_line = &line[start_str.len() + id.len()..];

            let valid = remaining_line.split(';').all(|set| {
                set.split(',').all(|sub_set| {
                    let sub_set = sub_set.trim().split(' ').collect::<Vec<_>>();
                    let count = sub_set.first().unwrap().parse::<u16>().unwrap();
                    let color = sub_set.last().unwrap();

                    match *color {
                        "red" => count <= RED_MAX,
                        "green" => count <= GREEN_MAX,
                        "blue" => count <= BLUE_MAX,
                        _ => panic!("color not recognized"),
                    }
                })
            });

            if valid {
                Some(id.parse::<u16>().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
enum Color<T>
where
    T: std::cmp::PartialOrd<T>,
{
    Red(T),
    Green(T),
    Blue(T),
}

// i could have just replaced T or before u16 with u32...
// but i wanted to make it generic xd... it was a bad idea lol
fn part_two<T>(input: &str) -> Vec<T>
where
    T: Mul + FromStr + PartialOrd + Default,
    <T as FromStr>::Err: Debug,
    <T as Mul>::Output: Mul<T>,
    Vec<T>: FromIterator<<<T as Mul>::Output as Mul<T>>::Output>,
{
    input
        .lines()
        .map(|line| {
            let remaining_line = line.split(':').nth(1).unwrap();

            remaining_line
                .split(';')
                .flat_map(|set| {
                    set.split(',')
                        .flat_map(|sub_set| {
                            let sub_set = sub_set.trim().split(' ').collect::<Vec<_>>();
                            let count = sub_set.first().unwrap().parse::<T>().unwrap();
                            let color = sub_set.last().unwrap();

                            match *color {
                                "red" => Some(Color::Red(count)),
                                "green" => Some(Color::Green(count)),
                                "blue" => Some(Color::Blue(count)),
                                _ => None,
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .fold(
                    (T::default(), T::default(), T::default()),
                    |(r, g, b), color: Color<T>| match color {
                        Color::Red(i) if i > r => (i, g, b),
                        Color::Green(i) if i > g => (r, i, b),
                        Color::Blue(i) if i > b => (r, g, i),
                        _ => (r, g, b),
                    },
                )
        })
        .map(|(r, g, b)| r * g * b)
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn part_one() {
        assert_eq!(vec![1, 2, 5], super::part_one(EXAMPLE))
    }

    #[test]
    fn part_two() {
        assert_eq!(vec![48, 12, 1560, 630, 36], super::part_two(EXAMPLE))
    }
}
