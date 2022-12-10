use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::separated_pair, IResult,
};

fn parse_u64(i: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(i)
}

type Range = (u64, u64);

fn range(i: &str) -> IResult<&str, Range> {
    separated_pair(parse_u64, tag("-"), parse_u64)(i)
}

type Pair = (Range, Range);

fn pair(i: &str) -> IResult<&str, Pair> {
    separated_pair(range, tag(","), range)(i)
}

fn parse(input: &str) -> impl Iterator<Item = Pair> + '_ {
    input.lines().flat_map(pair).map(|(_, r)| r)
}

fn main() {
    let input = include_str!("./input.txt");

    let p1 = parse(input)
        .filter(|p| match p {
            (a, b) if a.0 < b.0 => a.1 >= b.1,
            (a, b) if a.0 > b.0 => a.1 <= b.1,
            _ => true,
        })
        .count();

    let p2 = parse(input)
        .filter(|(a, b)| a.0 <= b.1 && a.1 >= b.0)
        .count();

    println!("p1: {p1}");
    println!("p2: {p2}");
}
