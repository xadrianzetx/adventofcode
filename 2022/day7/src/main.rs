use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Default, Debug)]
struct Directory {
    name: String,
    children: Vec<String>,
    files: Vec<File>,
}

#[derive(Default)]
struct FileSystem {
    curdir: String,
    fs: HashMap<String, Directory>    
}

impl From<&str> for Directory {
    fn from(name: &str) -> Self {
        Directory { name: String::from(name), ..Default::default() }
    }
}

impl Directory {
    fn get_size(&self, fs: &FileSystem) -> u32 {
        let mut total = 0;
        for child in &self.children {
            total += fs.fs.get(child).unwrap().get_size(fs);
        }
        for file in &self.files {
            total += file.size;
        }
        total
    }
}

impl FileSystem {
    fn sum_limited_size(&self) -> u32 {
        let mut total = 0;
        for (_, dir) in &self.fs {
            // println!("{:?}", dir);
            let size = dir.get_size(self);
            if size <= 100000 {
                total += size;
            }
        }
        total
    }
}

impl FileSystem {
    fn run_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split(' ').collect();
        match parts[1] {
            "cd" => {
                let newpath = match parts[2] {
                    "/" => {
                        String::from("/")
                    },
                    ".." => {
                        let pth = Path::new(&self.curdir);
                        String::from(pth.parent().unwrap().to_str().unwrap())
                    },
                    _ => {
                        let p = Path::new(&self.curdir);
                        String::from(p.join(parts[2]).as_path().to_str().unwrap())
                    }
                };
                
                if !self.fs.contains_key(&newpath) {
                    let newdir = Directory::from(newpath.as_str());
                    self.fs.insert(newpath.clone(), newdir);
                }
                self.curdir = newpath.clone();
                // println!("{}", self.curdir);
            },
            "ls" => (),
            _ => panic!()
        }
    }

    fn register_data(&mut self, data: &str) {
        let parts: Vec<&str> = data.split(' ').collect();
        match parts[0] {
            "dir" => {
                let p = Path::new(&self.curdir);
                let child = String::from(p.join(parts[1]).as_path().to_str().unwrap());
                let dir = self.fs.get_mut(&self.curdir).unwrap();
                dir.children.push(child);
            },
            _ => {
                let file = File {name: String::from(parts[1]), size: parts[0].parse().unwrap()};
                let dir = self.fs.get_mut(&self.curdir).unwrap();
                dir.files.push(file);
            }
        }
    }
}

fn main() {
    let mut fs = FileSystem::default();
    include_str!("../input").lines().for_each(|line| {
        if line.starts_with("$") {
            fs.run_command(line);
        } else {
            fs.register_data(line);
        }
    });
    println!("{}", fs.sum_limited_size());
}
