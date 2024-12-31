use std::cmp::min;

#[derive(Debug, PartialEq)]
enum BlockType {
    Free,
    Occupied,
}

// File id, file size.
struct File(usize, usize);

#[derive(Debug)]
struct Block {
    id: Option<usize>,
    block_type: BlockType,
    free_space: usize,
    occupied_space: usize,
    starting_address: usize,
}

impl Block {
    fn new_parametrized(size: usize, starting_address: usize, id: Option<usize>) -> Self {
        match id {
            Some(val) => Self {
                block_type: BlockType::Occupied,
                id: Some(val),
                free_space: 0,
                occupied_space: size,
                starting_address,
            },
            None => Self {
                block_type: BlockType::Free,
                id: None,
                free_space: size,
                occupied_space: 0,
                starting_address,
            },
        }
    }

    fn is_free(&self) -> bool {
        self.block_type == BlockType::Free
    }

    fn free(&mut self, size: usize) -> File {
        let freed = min(size, self.occupied_space);
        self.occupied_space -= freed;
        let file = File(self.id.unwrap(), freed);

        if self.occupied_space == 0 {
            self.id = None;
            self.block_type = BlockType::Free;
        }

        file
    }

    fn allocate(&mut self, file: File) -> Option<Block> {
        self.id = Some(file.0);
        self.block_type = BlockType::Occupied;
        self.occupied_space += file.1;

        if self.free_space - file.1 > 0 {
            return Some(Block::new_parametrized(
                self.free_space - self.occupied_space,
                self.starting_address + self.occupied_space,
                None,
            ));
        }

        None
    }

    fn checksum(&self) -> usize {
        if let BlockType::Free = self.block_type {
            return 0;
        }

        let mut checksum = 0;
        for i in self.starting_address..self.starting_address + self.occupied_space {
            checksum += i * self.id.unwrap();
        }
        checksum
    }
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Block>,
    free_ptr: usize,
    file_ptr: usize,
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut blocks = Vec::new();
        let mut addr = 0;

        let mut block_idx: usize = 0;
        let mut free_ptr = 0;
        let mut file_ptr = 0;

        for (id, chunk) in value.chars().collect::<Vec<char>>().chunks(2).enumerate() {
            let file_size = chunk[0].to_digit(10).unwrap() as usize;
            blocks.push(Block::new_parametrized(file_size, addr, Some(id)));
            addr += file_size;
            file_ptr = block_idx;
            block_idx += 1;

            if chunk.len() > 1 {
                let free_size = chunk[1].to_digit(10).unwrap() as usize;
                if free_size > 0 {
                    blocks.push(Block::new_parametrized(free_size, addr, None));
                    addr += free_size;

                    if free_ptr == 0 {
                        free_ptr = block_idx;
                    }
                    block_idx += 1;
                }
            }
        }

        Self {
            blocks,
            file_ptr,
            free_ptr,
        }
    }
}

impl Disk {
    fn fragmented_compact(&mut self) {
        loop {
            if self.file_ptr < self.free_ptr {
                break;
            }

            let file = self.get_file();
            if let Some(leftover) = self.write_file(file) {
                self.blocks.insert(self.free_ptr + 1, leftover);
            }

            self.next_file();
            self.next_free();
        }
    }

    fn defragmented_compact(&mut self) {
        let mut file_id = self.blocks[self.file_ptr].id.unwrap();
        loop {
            if file_id == 0 {
                break;
            }

            if self.find_free_space_for_current_file().is_some() {
                let file = self.get_file();
                if let Some(leftover) = self.write_file(file) {
                    self.blocks.insert(self.free_ptr + 1, leftover);
                }
            }

            file_id -= 1;
            self.find_file(file_id);
        }
    }

    fn find_free_space_for_current_file(&mut self) -> Option<()> {
        self.free_ptr = 0;
        let target = &self.blocks[self.file_ptr];

        loop {
            let block = &self.blocks[self.free_ptr];
            if block.is_free() && block.free_space >= target.occupied_space {
                return Some(());
            }

            self.free_ptr += 1;
            if self.free_ptr > self.file_ptr {
                break;
            }
        }
        None
    }

    fn next_free(&mut self) {
        loop {
            if self.blocks[self.free_ptr].is_free() {
                break;
            }
            self.free_ptr += 1;
        }
    }

    fn next_file(&mut self) {
        loop {
            if !self.blocks[self.file_ptr].is_free() {
                break;
            }
            self.file_ptr -= 1;
        }
    }

    fn find_file(&mut self, file_id: usize) {
        loop {
            let maybe_file = &self.blocks[self.file_ptr];
            if !maybe_file.is_free() && maybe_file.id.unwrap() == file_id {
                break;
            }
            self.file_ptr -= 1;
        }
    }

    fn get_file(&mut self) -> File {
        let size = self.blocks[self.free_ptr].free_space;
        let file_block = self.blocks.get_mut(self.file_ptr).unwrap();
        file_block.free(size)
    }

    fn write_file(&mut self, file: File) -> Option<Block> {
        let free_block = self.blocks.get_mut(self.free_ptr).unwrap();
        free_block.allocate(file)
    }

    fn get_checksum(&self) -> usize {
        self.blocks.iter().map(|b| b.checksum()).sum::<usize>()
    }
}

fn main() {
    let data = include_str!("../input");

    let mut fragmented_disk = Disk::from(data);
    fragmented_disk.fragmented_compact();
    println!("Part 1: {}", fragmented_disk.get_checksum());

    let mut defragmented_disk = Disk::from(data);
    defragmented_disk.defragmented_compact();
    println!("Part 2: {}", defragmented_disk.get_checksum());
}
