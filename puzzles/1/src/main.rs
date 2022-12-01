fn main() {
    let input = include_str!("./input.txt");

    let mut els = input
        .split("\n\n")
        .map(|i| {
            i.split("\n")
                .map(str::parse::<i32>)
                .filter_map(Result::ok)
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    els.sort();

    println!("Max: {}", els.last().unwrap());
    println!("Sum of Max 3: {:?}", els.iter().rev().take(3).sum::<i32>());
}
