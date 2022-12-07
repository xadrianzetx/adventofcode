use std::collections::HashMap;

#[derive(Default)]
struct FileSystem<'a> {
    current_dir: Vec<&'a str>,
    fs: HashMap<String, u32>,
}

impl<'a> FileSystem<'a> {
    fn new() -> FileSystem<'a> {
        Default::default()
    }

    fn sum_limited_size(&self) -> u32 {
        self.fs.values().filter(|v| v <= &&100000).sum()
    }

    fn find_smallest_to_delete(&self) -> u32 {
        let used = 70000000 - self.fs.get("/").unwrap();
        let required = 30000000 - used;
        *self.fs.values().filter(|v| v >= &&required).min().unwrap()
    }

    fn run_command(&mut self, command: &'a str) {
        let cmd: Vec<&str> = command.split(' ').collect();
        if let "cd" = cmd[1] {
            match cmd[2] {
                "/" => self.current_dir.push("/"),
                ".." => self.current_dir.truncate(self.current_dir.len() - 1),
                _ => self.current_dir.push(cmd[2]),
            };
        }
    }

    fn gather_output(&mut self, output: &str) {
        let data: Vec<&str> = output.split(' ').collect();
        if let Ok(file) = data[0].parse() {
            let mut abspath = String::new();
            self.current_dir.iter().for_each(|subdir| {
                abspath.push_str(subdir);
                self.fs
                    .entry(abspath.clone())
                    .and_modify(|size| *size += file)
                    .or_insert(file);
            })
        }
    }
}

fn main() {
    let mut fs = FileSystem::new();
    include_str!("../input").lines().for_each(|line| {
        if line.starts_with('$') {
            fs.run_command(line);
        } else {
            fs.gather_output(line);
        }
    });
    println!("Part1: {}", fs.sum_limited_size());
    println!("Part2: {}", fs.find_smallest_to_delete());
}
