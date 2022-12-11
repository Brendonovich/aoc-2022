use std::collections::HashSet;

fn find_marker<const UNIQUE: usize>(input: &str) -> usize {
    let mut marker = UNIQUE;

    let input = input.bytes().collect::<Vec<_>>();

    while input[(marker - UNIQUE)..marker]
        .iter()
        .collect::<HashSet<_>>()
        .len()
        != UNIQUE
    {
        marker += 1
    }

    marker
}

fn main() {
    let input = include_str!("./input.txt");

    println!("p1: {}", find_marker::<4>(input));
    println!("p2: {}", find_marker::<14>(input));
}

