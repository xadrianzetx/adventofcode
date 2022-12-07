use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
struct Directory {
    children: Vec<PathBuf>,
    files: Vec<u32>,
}

#[derive(Default)]
struct FileSystem {
    current_dir: PathBuf,
    fs: HashMap<PathBuf, Directory>,
}

impl Directory {
    fn get_size(&self, file_system: &FileSystem) -> u32 {
        self.children
            .iter()
            .map(|c| file_system.fs.get(c).unwrap().get_size(file_system))
            .sum::<u32>()
            + self.files.iter().sum::<u32>()
    }
}

impl FileSystem {
    fn sum_limited_size(&self) -> u32 {
        self.fs
            .values()
            .map(|d| d.get_size(self))
            .filter(|v| v <= &100000)
            .sum()
    }

    fn find_smallest_to_delete(&self) -> u32 {
        let used = 70000000 - self.fs.get(&PathBuf::from("/")).unwrap().get_size(self);
        let required = 30000000 - used;
        self.fs
            .values()
            .map(|d| d.get_size(self))
            .filter(|v| v >= &required)
            .min()
            .unwrap()
    }
}

impl FileSystem {
    fn run_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split(' ').collect();
        match parts[1] {
            "cd" => {
                let next_dir = match parts[2] {
                    "/" => PathBuf::from("/"),
                    ".." => self.current_dir.parent().unwrap().to_path_buf(),
                    _ => PathBuf::from(&self.current_dir).join(parts[2]),
                };

                if !self.fs.contains_key(&next_dir) {
                    self.fs.insert(next_dir.clone(), Directory::default());
                }
                self.current_dir = next_dir;
            }
            "ls" => (),
            _ => panic!(),
        }
    }

    fn register_data(&mut self, data: &str) {
        let parts: Vec<&str> = data.split(' ').collect();
        match parts[0] {
            "dir" => {
                let dir = self.fs.get_mut(&self.current_dir).unwrap();
                let path = PathBuf::from(&self.current_dir).join(parts[1]);
                dir.children.push(path);
            }
            _ => {
                let dir = self.fs.get_mut(&self.current_dir).unwrap();
                dir.files.push(parts[0].parse().unwrap());
            }
        }
    }
}

fn main() {
    let mut fs = FileSystem::default();
    include_str!("../input").lines().for_each(|line| {
        if line.starts_with('$') {
            fs.run_command(line);
        } else {
            fs.register_data(line);
        }
    });
    println!("Part1: {}", fs.sum_limited_size());
    println!("Part2: {}", fs.find_smallest_to_delete());
}
