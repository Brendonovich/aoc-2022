use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, not_line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
enum LsOutput {
    Dir,
    File { size: u64 },
}

fn ls_output(input: &str) -> IResult<&str, LsOutput> {
    alt((
        map(preceded(tag("dir "), alpha1), |_| LsOutput::Dir),
        map(
            separated_pair(complete::u64, tag(" "), not_line_ending),
            |(size, _)| LsOutput::File { size },
        ),
    ))(input)
}

#[derive(Debug)]
enum CdPath {
    Parent,
    Root,
    Folder(String),
}

fn cd_path(input: &str) -> IResult<&str, CdPath> {
    alt((
        map(tag(".."), |_| CdPath::Parent),
        map(tag("/"), |_| CdPath::Root),
        map(alpha1, |folder: &str| CdPath::Folder(folder.to_string())),
    ))(input)
}

#[derive(Debug)]
enum Command {
    Cd { path: CdPath },
    Ls { outputs: Vec<LsOutput> },
}

fn command(input: &str) -> IResult<&str, Command> {
    preceded(
        tag("$ "),
        alt((
            map(preceded(tag("cd "), cd_path), |path| Command::Cd { path }),
            map(
                preceded(tag("ls\n"), separated_list0(newline, ls_output)),
                |outputs| Command::Ls { outputs },
            ),
        )),
    )(input)
}

fn parse(input: &str) -> Vec<Command> {
    separated_list0(newline, command)(input)
        .map(|(_, v)| v)
        .unwrap_or_default()
}

fn get_sizes(commands: Vec<Command>) -> HashMap<String, u64> {
    let mut working_dir_path = VecDeque::new();
    let mut sizes = HashMap::new();

    for command in commands {
        match command {
            Command::Cd { path } => match path {
                CdPath::Root => {
                    working_dir_path.clear();
                }
                CdPath::Parent => {
                    working_dir_path.pop_back();
                }
                CdPath::Folder(folder) => {
                    working_dir_path.push_back(folder);
                }
            },
            Command::Ls { outputs } => {
                for output in outputs {
                    match output {
                        LsOutput::File { size } => {
                            for i in 0..working_dir_path.len() + 1 {
                                let current_size = sizes
                                    .entry(
                                        working_dir_path
                                            .range(..i)
                                            .cloned()
                                            .collect::<Vec<_>>()
                                            .join("/"),
                                    )
                                    .or_insert(0);
                                *current_size = *current_size + size;
                            }
                        }
                        _ => {}
                    };
                }
            }
        }
    }

    sizes
}

fn main() {
    let input = include_str!("./input.txt");

    let sizes = get_sizes(parse(input));

    println!(
        "p1: {}",
        sizes
            .values()
            .filter_map(|&size| (size <= 100000).then_some(size))
            .sum::<u64>()
    );

    println!("p2: {:?}", {
        let used = sizes.values().cloned().max().unwrap_or_default();

        let must_reclaim = used + 30000000 - 70000000;

        sizes.values().fold(None, |acc, &current| match acc {
            None if current >= must_reclaim => Some(current),
            Some(acc) if current < acc && current >= must_reclaim => Some(current),
            _ => acc,
        })
    })
}
