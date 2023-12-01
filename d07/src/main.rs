use std::fmt::{self, Display, Formatter};

use common::read_input;

use crate::consts::TOTAL_DISK_SPACE;

mod consts {
    pub const MIN_SIZE_THRESHOLD: u32 = 100_000;
    pub const TOTAL_DISK_SPACE: u32 = 70_000_000;
    pub const TOTAL_SPACE_NEEDED: u32 = 30_000_000;
}

#[derive(Debug)]
enum Node {
    File {
        size: u32,
    },
    Folder {
        files: Vec<(String, Node)>,
        cached_size: Option<u32>,
    },
}

// Generate Node::File from a u32
impl From<u32> for Node {
    fn from(size: u32) -> Self {
        Self::File { size }
    }
}

// Generate Node::Folder from a vector of nodes
impl From<Vec<(String, Node)>> for Node {
    fn from(mut value: Vec<(String, Node)>) -> Self {
        #[cfg(debug_assertions)]
        {
            value.sort_by_key(|(s, _)| s.to_owned());
        }
        Self::Folder {
            files: value,
            cached_size: None,
        }
    }
}

impl Node {
    fn parse(input: &str) -> Self {
        let mut lines = input
            .lines()
            .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
            .peekable();

        let mut stack = Vec::new();

        let mut cur_name: String = String::new();
        let mut cur_node: Vec<(String, Node)> = Vec::new();

        // this syntax over lines.by_ref() solves a borrow checker issue
        while let Some(line) = lines.next() {
            match line[..] {
                ["$", "cd", "/"] => {
                    if !stack.is_empty() {
                        panic!("Unexpected cd to root");
                    }
                }
                ["$", "cd", ".."] => {
                    let new = (cur_name, cur_node.into());
                    (cur_name, cur_node) = stack.pop().expect("should be a node");
                    cur_node.push(new);
                }
                ["$", "cd", target] => {
                    stack.push((cur_name, cur_node));
                    cur_name = target.to_owned();
                    cur_node = Vec::new();
                }
                // grabs the next arguments that do not start with `$`
                ["$", "ls"] => {
                    while let Some(args) = lines.next_if(|v| v[0] != "$") {
                        match args[..] {
                            // ignoring `dir` output because we're going to
                            // uncover that via `cd` anyways
                            ["dir", _] => (),
                            [size, file] => {
                                let node = size.parse::<u32>().unwrap().into();
                                cur_node.push((file.into(), node));
                            }
                            [..] => panic!("invalid listing"),
                        }
                    }
                }
                ["$", ..] => panic!("Invalid command! expected `cd <target>` or `ls`"),
                [..] => panic!("expected a command"),
            };
        }

        // simulate a couple `cd ..`s
        while !stack.is_empty() {
            let new = (cur_name, cur_node.into());
            (cur_name, cur_node) = stack.pop().expect("should be a node");
            cur_node.push(new);
        }

        // by this point, cur_node should be the root node
        cur_node.into()
    }

    fn fmt_recursive(&self, f: &mut Formatter<'_>, name: &str, indent: u32) -> fmt::Result {
        for _ in 0..indent {
            write!(f, "  ")?;
        }

        match self {
            Node::File { size } => writeln!(f, "- {name} ({size})")?,
            Node::Folder { files, .. } => {
                writeln!(f, "- {name}/")?;
                let indent = indent + 1;
                for (name, node) in files {
                    node.fmt_recursive(f, name, indent)?;
                }
            }
        };

        Ok(())
    }
}

impl Display for Node {
    fn fmt<'a>(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_recursive(f, "", 0)
    }
}

impl Node {
    fn size_cached(&mut self) -> u32 {
        match self {
            Node::File { size } => *size,
            Node::Folder {
                files: _,
                cached_size: Some(n),
            } => *n,
            Node::Folder { files, cached_size } => {
                // in the case that it wasn't cached, we want to get the size, update the cache,
                // and return that cached value
                let resp = files.iter_mut().map(|n| n.1.size_cached()).sum();
                *cached_size.insert(resp)
            }
        }
    }
}

impl Node {
    /// Sums the size of each directory with a size lower than the given threshold.
    fn sum_sizes_beneath(&mut self, threshold: u32) -> u32 {
        let mut stack = vec![self];
        let mut resp = 0;
        while let Some(node) = stack.pop() {
            // undoubtedly bad code, but I'm tired of thinking about it
            // don't bother optimizing it
            if matches!(node, Self::Folder { .. }) {
                let size = node.size_cached();
                if size < threshold {
                    resp += size;
                };
            }

            if let Self::Folder { ref mut files, .. } = node {
                let nodes = files.iter_mut().map(|sn| &mut sn.1);
                stack.extend(nodes);
            }
        }
        resp
    }

    /// Gets the smallest directory whose size would free up enough
    /// space for an update with this size.
    ///
    /// - update_size: size of the update
    /// - max_size:    maximum size of this root folder
    fn smallest_directory_freeing(&mut self, update_size: u32, max_size: u32) -> u32 {
        // minimum size of a folder to be a viable deletion candidate
        let min_size = self.size_cached() - (max_size - update_size);
        let mut resp = max_size;
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            if matches!(node, Self::Folder { .. }) {
                let size = node.size_cached();
                if min_size < size && size < resp {
                    resp = size;
                };
            }

            if let Self::Folder { ref mut files, .. } = node {
                let nodes = files.iter_mut().map(|sn| &mut sn.1);
                stack.extend(nodes);
            }
        }
        resp
    }
}

// Sample output: 95437, 24933642
fn main() {
    let input = read_input!();
    let mut node = Node::parse(&input);
    println!(
        "{}, {}",
        node.sum_sizes_beneath(consts::MIN_SIZE_THRESHOLD),
        node.smallest_directory_freeing(consts::TOTAL_SPACE_NEEDED, TOTAL_DISK_SPACE)
    );
}
