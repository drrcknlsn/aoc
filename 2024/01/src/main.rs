use std::error::Error;
use std::fs;
use std::iter::zip;

use nom::character::complete::{ digit1, multispace1, newline };
use nom::IResult;
use nom::multi::many1;

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, left) = digit1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, right) = digit1(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (
        left.parse().unwrap(),
        right.parse().unwrap(),
    )))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(parse_line)(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    // Part 1
    match parse_input(&input) {
        Ok((_, pairs)) => {
            let mut left: Vec<i32> = vec![];
            let mut right: Vec<i32> = vec![];

            for (l, r) in pairs {
                left.push(l);
                right.push(r);
            }

            left.sort();
            right.sort();

            let mut sum_of_distances = 0;

            for (l, r) in zip(left, right) {
                sum_of_distances += (l - r).abs();
            }

            println!("Part 1: {sum_of_distances}");
        },
        Err(_) => {
            println!("Failed to parse input");
        },
    };

    // Part 2
    match parse_input(&input) {
        Ok((_, pairs)) => {
            let mut left: Vec<i32> = vec![];
            let mut right: Vec<i32> = vec![];

            for (l, r) in pairs {
                left.push(l);
                right.push(r);
            }

            let mut occurrences = std::collections::HashMap::new();

            for r in right {
                occurrences.entry(r)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }

            let mut similarity_score = 0;

            for l in left {
                similarity_score += l * occurrences.get(&l).unwrap_or(&0);
            }

            println!("Part 2: {similarity_score}");
        },
        Err(_) => {
            println!("Failed to parse input");
        },
    };

    Ok(())
}
