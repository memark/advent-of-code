#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use itertools::Itertools;
use std::{collections::HashMap, fs, str::FromStr, string::ParseError};

fn main() {
    println!("Part 1: {}", solve_part_1(&file("input")));
    println!("Part 2: {}", solve_part_2(&file("input")));
}

fn solve_part_1(input: &str) -> u32 {
    let entries = parse_entries(input);
    let dirs = calculate_directory_sizes(&entries);
    let dirs_incl = aggregate_directory_sizes(&dirs);
    let sum = dirs_incl.values().filter(|x| **x <= 100_000).sum::<u32>();

    sum
}

fn solve_part_2(input: &str) -> u32 {
    let entries = parse_entries(input);
    let dirs = calculate_directory_sizes(&entries);
    let dirs_incl = aggregate_directory_sizes(&dirs);
    dbg!(&dirs_incl);

    let total_space = 70000000;
    let used_space = dirs_incl.get("/").unwrap();
    let available_space = total_space - used_space;
    let needed_available_space = 30000000;
    let needed_additional_available_space = needed_available_space - available_space;
    dbg!(needed_additional_available_space);

    let answer = dirs_incl
        .iter()
        .filter(|(path, space)| **space >= needed_additional_available_space)
        .sorted_by_key(|&(path, space)| space)
        .next()
        .unwrap()
        .1;

    *answer
}

fn parse_entries(input: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut last_file_or_dir: Vec<FileOrDir> = Vec::new();
    for l in input.lines() {
        if let Some(cmd) = l.strip_prefix("$ ") {
            if !last_file_or_dir.is_empty() {
                entries.push(Entry::Output(last_file_or_dir));
                last_file_or_dir = Vec::new();
            }

            entries.push(Entry::Command(cmd.parse().unwrap()))
        } else {
            last_file_or_dir.push(l.parse().unwrap());
        }
    }
    if !last_file_or_dir.is_empty() {
        entries.push(Entry::Output(last_file_or_dir));
    }
    entries
}

fn calculate_directory_sizes(entries: &Vec<Entry>) -> HashMap<String, u32> {
    let mut hm: HashMap<String, u32> = HashMap::new();
    let mut curr_dir = vec![];
    for entry in entries {
        match entry {
            Entry::Command(command) => match command {
                Command::Cd(path) => curr_dir.push(path.to_owned()),
                Command::CdUp => {
                    curr_dir.pop().unwrap();
                }
                Command::CdRoot => {
                    curr_dir = vec![];
                }
                Command::Ls => {}
            },
            Entry::Output(outputs) => {
                for output in outputs {
                    match output {
                        FileOrDir::File(size, name) => {
                            let joined = curr_dir.join("/");
                            let path = format!("/{joined}");
                            *hm.entry(path).or_insert(0) += size;
                        }
                        FileOrDir::Dir(_) => {
                            let joined = curr_dir.join("/");
                            let path = format!("/{joined}");
                            *hm.entry(path).or_insert(0) += 0;
                        }
                    }
                }
            }
        }
    }

    hm
}

fn aggregate_directory_sizes(sizes: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut hm: HashMap<String, u32> = HashMap::new();

    for (s, u) in sizes {
        let aggr_size = sizes
            .keys()
            .filter(|path| path.starts_with(s))
            .map(|x| sizes.get(x).unwrap())
            .sum();

        let res = hm.insert(s.to_owned(), aggr_size);
        if res.is_some() {
            panic!();
        }
    }

    hm
}

// fn aggregate_directory_size(sizes: &HashMap<String, u32>, directory: String) -> u32 {
//     let relevant_dirs = sizes
//         .keys()
//         .filter(|path| path.starts_with(&directory))
//         .collect_vec();
//     if relevant_dirs.len() == 1 {
//         return *sizes.get(relevant_dirs[0]).unwrap();
//     }
//     relevant_dirs.iter().fold(0, |acc, rd| {
//         acc + aggregate_directory_size(sizes, rd.to_owned().to_owned())
//     })
// }

enum FileOrDir2 {
    File2(File),
    Dir2(Dir),
}

struct File {
    size: u32,
    name: String,
}

struct Dir {
    name: String,
    members: Vec<FileOrDir2>,
}

#[derive(Debug)]
enum Entry {
    Command(Command),
    Output(Vec<FileOrDir>),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    CdUp,
    CdRoot,
    Ls,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ls" {
            Ok(Command::Ls)
        } else if s.starts_with("cd") {
            let (_cd, arg) = s.split_once(' ').unwrap();
            match arg {
                "/" => Ok(Command::CdRoot),
                ".." => Ok(Command::CdUp),
                _ => Ok(Command::Cd(arg.to_owned())),
            }
        } else {
            panic!();
        }
    }
}

#[derive(Debug)]
enum FileOrDir {
    File(u32, String),
    Dir(String),
}

impl FromStr for FileOrDir {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(path) = s.strip_prefix("dir ") {
            Ok(FileOrDir::Dir(path.to_owned()))
        } else {
            let (size, name) = s.split_once(' ').unwrap();
            Ok(FileOrDir::File(size.parse().unwrap(), name.to_owned()))
        }
    }
}

fn file(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1(&file("example_1")), 95437);

        let entries = parse_entries(&file("example_1"));
        let dirs = calculate_directory_sizes(&entries);
        let dirs_incl = aggregate_directory_sizes(&dirs);

        assert_eq!(*dirs_incl.get("/a/e").unwrap(), 584);
        assert_eq!(*dirs_incl.get("/a").unwrap(), 94853);
        assert_eq!(*dirs_incl.get("/d").unwrap(), 24933642);
        assert_eq!(*dirs_incl.get("/").unwrap(), 48381165);
    }

    #[test]
    fn part_1_input() {
        assert_eq!(solve_part_1(&file("input")), 1423358);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(&file("example_1")), 24933642);
    }

    #[ignore]
    #[test]
    fn part_2_input() {
        assert_eq!(solve_part_2(&file("input")), todo!());
    }
}
