advent_of_code::solution!(7);

use std::collections::HashMap;

type Path = String;

#[derive(Clone, Debug)]
struct Dirent {
    path: Path,
    node: INode,
}

#[derive(Clone, Debug)]
enum INode {
    Dir,
    File(usize),
}

struct Filesystem {
    directories: HashMap<Path, Vec<Dirent>>,
}

fn form_path(path: &[&str]) -> Path {
    format!("/{}", path.join("/"))
}

impl Filesystem {
    fn new(input: &str) -> Self {
        let mut directories: HashMap<Path, Vec<Dirent>> = HashMap::new();
        let mut path = Vec::new();
        for line in input.lines() {
            if let Some(arg) = line.strip_prefix("$ cd ") {
                match arg {
                    ".." => {
                        path.pop();
                    }
                    "/" => path.clear(),
                    _ => path.push(arg),
                }
                continue;
            }
            if line.starts_with('$') {
                continue;
            }

            let full_path = form_path(&path);

            let (size_type, name) = line.split_once(' ').unwrap();
            let node = if size_type == "dir" {
                INode::Dir
            } else {
                INode::File(size_type.parse().unwrap())
            };

            path.push(name);
            let file = Dirent {
                path: form_path(&path),
                node,
            };
            path.pop();

            directories.entry(full_path.clone()).or_default();

            directories.entry(full_path).and_modify(|e| e.push(file));
        }
        dbg!(&directories);
        Self { directories }
    }

    fn directory_size(&self, p: &Path) -> usize {
        let mut result = 0;
        for entry in self.directories.get(p).unwrap().iter() {
            result += match entry.node {
                INode::Dir => self.directory_size(&entry.path),
                INode::File(size) => size,
            }
        }
        result
    }

    fn directory_sizes(&self) -> Vec<usize> {
        self.directories
            .keys()
            .map(|p| self.directory_size(p))
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let fs = Filesystem::new(input);
    Some(fs.directory_sizes().iter().filter(|&s| *s <= 100000).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
