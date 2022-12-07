use std::collections::HashMap;

use itertools::Itertools;
use serde_scan::scan;

#[derive(Debug)]
enum DirEntry {
    File(i32),
    Directory(HashMap<String, DirEntry>),
}

impl DirEntry {
    fn new_dir() -> Self {
        DirEntry::Directory(HashMap::new())
    }

    fn unwrap_directory(&self) -> &HashMap<String, DirEntry> {
        if let DirEntry::Directory(d) = self {
            d
        } else {
            panic!()
        }
    }

    fn unwrap_directory_mut(&mut self) -> &mut HashMap<String, DirEntry> {
        if let DirEntry::Directory(d) = self {
            d
        } else {
            panic!()
        }
    }

    fn insert_directory(&mut self, name: &str) {
        if let DirEntry::Directory(d) = self {
            d.insert(name.to_string(), Self::new_dir());
        } else {
            panic!();
        }
    }

    fn insert_file(&mut self, name: &str, size: i32) {
        if let DirEntry::Directory(d) = self {
            d.insert(name.to_string(), DirEntry::File(size));
        } else {
            panic!()
        }
    }

    fn get_size(&self) -> i32 {
        match self {
            DirEntry::File(size) => *size,
            DirEntry::Directory(dir) => dir.iter().fold(0, |acc, item| acc + item.1.get_size()),
        }
    }
}

#[derive(Debug)]

struct FS {
    data: DirEntry,
}

impl FS {
    fn new() -> Self {
        FS {
            data: DirEntry::Directory(HashMap::new()),
        }
    }

    fn path_vec(path: &str) -> Vec<&str> {
        path.split("/").filter(|str| !str.is_empty()).collect_vec()
    }

    fn get_directory(&mut self, path: &str) -> &mut DirEntry {
        if path == "" || path == "/" {
            return &mut self.data;
        }
        let route = FS::path_vec(path);
        let mut cwd = &mut self.data;
        for p in route {
            cwd = cwd.unwrap_directory_mut().get_mut(p).unwrap();
        }
        cwd
    }

    fn add_directory(&mut self, path: &str, dirname: &str) {
        let cwd = self.get_directory(path);
        cwd.insert_directory(dirname);
    }

    fn add_file(&mut self, path: &str, filename: &str, size: i32) {
        let cwd = self.get_directory(path);
        cwd.insert_file(&filename, size);
    }
}

fn size_of_small_dirs(dir: &DirEntry) -> i32 {
    let mut sum = 0;
    for (_, entry) in dir.unwrap_directory() {
        if let DirEntry::Directory(_) = entry {
            let size = entry.get_size();
            if size <= 100000 {
                sum += size;
            }
            sum += size_of_small_dirs(entry);
        }
    }
    sum
}

fn smallest_dir_large_enough(
    dir: &DirEntry,
    target: i32,
    mut smallest_so_far: Option<i32>,
) -> Option<i32> {
    for (_, entry) in dir.unwrap_directory() {
        if let DirEntry::Directory(_) = entry {
            let size = entry.get_size();
            if size >= target {
                if smallest_so_far.is_none() {
                    smallest_so_far = Some(size);
                } else if smallest_so_far.unwrap() > size {
                    smallest_so_far = Some(size);
                }
            }
            smallest_so_far = smallest_dir_large_enough(entry, target, smallest_so_far);
        }
    }
    smallest_so_far
}

fn main() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();

    let mut cwd = String::from("/");
    let mut fs = FS::new();
    for line in input.lines() {
        if line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("$ cd") {
            cwd = match scan!("$ cd {}" <- line).unwrap() {
                "/" => String::from("/"),
                ".." => {
                    let mut dirs = cwd.split("/").collect_vec();
                    dirs.pop();
                    format!("/{}", dirs.join("/"))
                }
                name => {
                    if cwd == "/" {
                        format!("/{}", name)
                    } else {
                        format!("{}/{}", cwd, name)
                    }
                }
            };
            continue;
        }

        let (dir_or_size, name): (String, String) = scan!("{} {}" <- line).unwrap();
        if name == "/" {
            continue;
        }
        if dir_or_size == "dir" {
            fs.add_directory(&cwd, &name);
        } else {
            fs.add_file(&cwd, &name, dir_or_size.parse().unwrap());
        }
    }

    println!(
        "The total size of all directories of size 100000 or less is {}.",
        size_of_small_dirs(&fs.data)
    );
    println!(
        "The size of the smallest directory that would free up enough space on disk is {} in size.",
        smallest_dir_large_enough(&fs.data, 30000000 - (70000000 - fs.data.get_size()), None)
            .unwrap()
    );
}
