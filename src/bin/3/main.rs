fn main() {
    let rucksacks = include_str!("./input.txt").lines().collect::<Vec<_>>();

    let p1 = rucksacks
        .iter()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|comps| (comps.0.chars(), comps.1.chars()))
        .map(|(chars0, chars1)| {
            chars0
                .clone()
                .into_iter()
                .find(|char0| chars1.clone().into_iter().any(|char1| *char0 == char1))
        })
        .flatten()
        .map(|c| {
            (c.to_ascii_lowercase() as i32) - ('a' as i32 - 1) + (26 * c.is_uppercase() as i32)
        })
        .sum::<i32>();

    let p2 = rucksacks.chunks(3);

    println!("Part 1: {p1}");
}
