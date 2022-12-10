use std::{collections::VecDeque, str::FromStr};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::*,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(input)
}

struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        map(
            tuple((
                tag("move "),
                parse_usize,
                tag(" from "),
                parse_usize,
                tag(" to "),
                parse_usize,
            )),
            |(_, count, _, from, _, to)| Self {
                count,
                from: from - 1,
                to: to - 1,
            },
        )(s)
        .map(|(_, i)| i)
        .map_err(|_| ())
    }
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().flat_map(str::parse)
}

fn parse_state_base_line(input: &str) -> IResult<&str, usize> {
    map(
        separated_list0(char(' '), delimited(char(' '), digit1, char(' '))),
        |v| v.len(),
    )(input)
}

fn parse_state_stack_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list0(
        char(' '),
        alt((
            map(delimited(char('['), anychar, char(']')), |c| Some(c)),
            map(tag("   "), |_| None),
        )),
    )(input)
}

fn parse_state(input: &str) -> Vec<VecDeque<char>> {
    let Some((_, stack_count)) = input.lines().last().map(parse_state_base_line).and_then(Result::ok) else { return vec![]; };

    let mut state = vec![VecDeque::new(); stack_count];

    input
        .lines()
        .take(input.lines().count() - 1)
        .flat_map(parse_state_stack_line)
        .for_each(|(_, line)| {
            line.iter().enumerate().for_each(|(index, c)| {
                let Some(c) = c else { return; };

                state[index].push_back(*c);
            })
        });

    dbg!(&state);

    state
}

fn main() {
    let input = include_str!("./input.txt");

    let Some((state_str, instructions)) = input.split_once("\n\n") else { return; };

    let mut state = parse_state(state_str);

    for instruction in parse_instructions(instructions) {
        for _ in 0..instruction.count {
            let Some(c) = state[instruction.from].pop_front() else { return };

            state[instruction.to].push_front(c);
        }
    }

    println!(
        "p1: {}",
        state
            .iter()
            .flat_map(|v| v.front().cloned())
            .collect::<String>()
    );

    let mut state = parse_state(state_str);

    for instruction in parse_instructions(instructions) {
        let a = state[instruction.from]
            .drain(0..instruction.count)
            .collect::<Vec<_>>();

        a.into_iter()
            .rev()
            .for_each(|c| state[instruction.to].push_front(c));
    }

    println!(
        "p2: {}",
        state
            .iter()
            .flat_map(|v| v.front().cloned())
            .collect::<String>()
    )
}
