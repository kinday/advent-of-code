use std::{collections::HashMap, str::FromStr};

const TOTAL_SPACE: i32 = 70000000;
const REQUIRED_SPACE: i32 = 30000000;

#[derive(Debug)]
struct TerminalLineParseError;

#[derive(Debug, PartialEq, Eq)]
enum TerminalLine {
    ChangeDirInput(String),
    DirectoryOutput(String),
    FileOutput(String, i32),
    ListInput,
}

#[derive(Debug)]
struct FileParseError;

#[derive(Debug)]
struct File {
    size: i32,
}

#[derive(Debug)]
enum FileTreeEntry {
    DirEntry(HashMap<String, FileTreeEntry>),
    FileEntry(File),
}

#[derive(Debug)]
struct FileTreeParseError;

#[derive(Debug)]
struct FileTree {
    value: FileTreeEntry,
}

impl FromStr for TerminalLine {
    type Err = TerminalLineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ") {
            if let Some(directory_name) = s.strip_prefix("$ cd ") {
                Ok(TerminalLine::ChangeDirInput(directory_name.to_string()))
            } else if s.starts_with("$ ls") {
                Ok(TerminalLine::ListInput)
            } else {
                Err(TerminalLineParseError)
            }
        } else if let Some(directory_name) = s.strip_prefix("dir ") {
            Ok(TerminalLine::DirectoryOutput(directory_name.to_string()))
        } else {
            let (size, name) = s.split_once(" ").ok_or(TerminalLineParseError)?;

            let name = name.to_string();
            let size = size.parse::<i32>().map_err(|_| TerminalLineParseError)?;

            Ok(TerminalLine::FileOutput(name, size))
        }
    }
}

impl FromStr for FileTree {
    type Err = FileTreeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree = HashMap::new();
        let mut path = Vec::new();
        for line in s.lines() {
            if let Ok(line) = line.parse::<TerminalLine>() {
                match line {
                    TerminalLine::ChangeDirInput(x) => {
                        if x == "/" {
                            path.clear();
                        } else if x == ".." {
                            path.pop();
                        } else {
                            path.push(x);
                        }
                    }
                    TerminalLine::DirectoryOutput(x) => {
                        path.iter()
                            .try_fold(&mut tree, |subtree, key| match subtree.get_mut(key) {
                                Some(FileTreeEntry::DirEntry(x)) => Ok(x),
                                _ => Err(FileTreeParseError),
                            })?
                            .insert(x, FileTreeEntry::DirEntry(HashMap::new()));
                    }
                    TerminalLine::FileOutput(name, size) => {
                        path.iter()
                            .try_fold(&mut tree, |subtree, key| match subtree.get_mut(key) {
                                Some(FileTreeEntry::DirEntry(x)) => Ok(x),
                                _ => Err(FileTreeParseError),
                            })?
                            .insert(name.clone(), FileTreeEntry::FileEntry(File { size }));
                    }
                    _ => {}
                }
            }
        }
        Ok(FileTree {
            value: FileTreeEntry::DirEntry(tree),
        })
    }
}

impl FileTreeEntry {
    fn size(&self) -> i32 {
        match self {
            FileTreeEntry::DirEntry(dir) => dir.values().map(|x| x.size()).sum(),
            FileTreeEntry::FileEntry(file) => file.size,
        }
    }
}

fn get_filetree_stats(parent: String, entry: &FileTreeEntry) -> HashMap<String, i32> {
    match entry {
        FileTreeEntry::DirEntry(subdir) => {
            let mut stats: HashMap<String, i32> = HashMap::new();

            // Insert self
            stats.insert(parent.clone(), entry.size());

            for (name, entry) in subdir {
                if let FileTreeEntry::DirEntry(_) = entry {
                    let key = format!("{}/{}", parent, name);
                    let value = entry.size();

                    // Insert child
                    stats.insert(key.clone(), value);

                    for (ck, cv) in get_filetree_stats(key, entry) {
                        // Insert child’s children
                        stats.insert(ck, cv);
                    }
                }
            }
            stats
        }
        _ => HashMap::new(),
    }
}

fn solve_first(input: &String) -> String {
    let tree = input.parse::<FileTree>().unwrap();
    let stats = get_filetree_stats(String::from("<root>"), &tree.value);
    let sum: i32 = stats.values().filter(|x| x < &&100000).sum();
    sum.to_string()
}

fn solve_second(input: &String) -> String {
    let tree = input.parse::<FileTree>().unwrap();
    let stats = get_filetree_stats(String::from("<root>"), &tree.value);
    let free_space = TOTAL_SPACE - tree.value.size();
    let needed_space = REQUIRED_SPACE - free_space;
    let min: i32 = stats
        .values()
        .filter(|x| x >= &&needed_space)
        .min()
        .unwrap()
        .to_owned();
    min.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (solve_first(&input), solve_second(&input))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_terminal_line_parse_cd() {
        let actual = "$ cd a".parse::<TerminalLine>().unwrap();
        assert_eq!(actual, TerminalLine::ChangeDirInput(String::from("a")));
    }

    #[test]
    fn test_terminal_line_parse_ls() {
        let actual = "$ ls".parse::<TerminalLine>().unwrap();
        assert_eq!(actual, TerminalLine::ListInput);
    }

    #[test]
    fn test_terminal_line_parse_dir() {
        let actual = "dir a".parse::<TerminalLine>().unwrap();
        assert_eq!(actual, TerminalLine::DirectoryOutput(String::from("a")));
    }

    #[test]
    fn test_terminal_line_parse_file() {
        let actual = "1234 foo.txt".parse::<TerminalLine>().unwrap();
        assert_eq!(
            actual,
            TerminalLine::FileOutput(String::from("foo.txt"), 1234)
        );
    }
}
