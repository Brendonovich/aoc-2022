use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split("")
                .flat_map(str::parse::<u8>)
                .enumerate()
                .map(move |(x, n)| ((x, y), n))
        })
        .collect::<HashMap<_, _>>();

    let Some((width, height)) = grid.keys().max().cloned().map(|(x, y)| (x + 1, y + 1)) else { return; };

    let p1 = {
        let mut visible = {
            let mut ret = HashSet::new();

            for x in 0..width {
                ret.insert((x, 0));
                ret.insert((x, height - 1));
            }

            for y in 0..height {
                ret.insert((0, y));
                ret.insert((width - 1, y));
            }

            ret
        };

        let fold_body = |mut acc: Vec<_>, (index, point)| {
            let current_height = *grid.get(&point).unwrap();

            match acc.last() {
                Some(&(highest, _)) => {
                    if current_height > highest {
                        acc.push((current_height, index));
                    }
                }
                None => acc.push((current_height, index)),
            }

            acc
        };

        for y in 0..height {
            let fold = |i: &mut dyn Iterator<Item = _>| {
                i.map(|x| (x, (x, y)))
                    .fold(vec![], fold_body)
                    .into_iter()
                    .map(|(_, x)| (x, y))
            };

            visible.extend(fold(&mut (0..width)));
            visible.extend(fold(&mut (0..width).rev()));
        }

        for x in 0..width {
            let fold = |i: &mut dyn Iterator<Item = _>| {
                i.map(|y| (y, (x, y)))
                    .fold(vec![], fold_body)
                    .into_iter()
                    .map(|(_, y)| (x, y))
            };

            visible.extend(fold(&mut (0..height)));
            visible.extend(fold(&mut (0..height).rev()));
        }

        visible.len()
    };

    let p2 = {
        grid.iter()
            .map(|((x, y), tree)| {
                let mut up = 0;

                for y in (0..*y).rev() {
                    match grid.get(&(*x, y)) {
                        Some(point) => {
                            up += 1;

                            if point >= tree {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                let mut down = 0;

                for y in (y + 1)..height {
                    match grid.get(&(*x, y)) {
                        Some(point) => {
                            down += 1;

                            if point >= tree {
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                let mut left = 0;

                for x in (0..*x).rev() {
                    match grid.get(&(x, *y)) {
                        Some(point) => {
                            left += 1;

                            if point >= tree {
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                let mut right = 0;

                for x in (x + 1)..width {
                    match grid.get(&(x, *y)) {
                        Some(point) => {
                            right += 1;

                            if point >= tree {
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                up * down * left * right
            })
            .max()
            .unwrap_or_default()
    };

    dbg!(width, height);
    println!("p1: {p1}");
    println!("p2: {p2}");
}
