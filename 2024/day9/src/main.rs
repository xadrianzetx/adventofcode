#[derive(Debug, PartialEq)]
enum BlockType {
    Free,
    Occupied,
}

#[derive(Debug)]
struct Block {
    type_: BlockType,
    id: Option<usize>,
    free: usize,
    occupied: usize,
    addr: usize,
}

impl Block {
    fn new_free(size: usize, addr: usize) -> Self {
        Self {
            type_: BlockType::Free,
            id: None,
            free: size,
            occupied: 0,
            addr,
        }
    }

    fn new_occupied(id: usize, size: usize, addr: usize) -> Self {
        Self {
            type_: BlockType::Occupied,
            id: Some(id),
            free: 0,
            occupied: size,
            addr,
        }
    }

    fn checksum(&self) -> usize {
        if let BlockType::Free = self.type_ {
            return 0;
        }

        let mut checksum = 0;
        for i in self.addr..self.addr + self.occupied {
            checksum += i * self.id.unwrap();
        }
        checksum
    }
}

fn build_blocks(data: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut file = true;
    let mut tot_size = 0;
    let mut next_id = 0;

    for chr in data.chars() {
        let size = chr.to_digit(10).unwrap() as usize;
        if file {
            let block = Block::new_occupied(next_id, size, tot_size);
            blocks.push(block);
            file = false;
            next_id += 1;
        } else {
            let block = Block::new_free(size, tot_size);
            blocks.push(block);
            file = true;
        }
        tot_size += size;
    }

    blocks
}

fn get_free_block(blocks: &[Block]) -> usize {
    for (ptr, block) in blocks.iter().enumerate() {
        if let BlockType::Free = block.type_ {
            return ptr;
        }
    }
    unreachable!();
}

fn get_file_block(blocks: &[Block]) -> usize {
    for (ptr, block) in blocks.iter().enumerate().rev() {
        if let BlockType::Occupied = block.type_ {
            return ptr;
        }
    }
    unreachable!();
}

fn fragment(blocks: &mut Vec<Block>) {
    loop {
        let free_ptr = get_free_block(blocks);
        let file_ptr = get_file_block(blocks);
        if blocks[file_ptr].addr < blocks[free_ptr].addr {
            break;
        }

        blocks[free_ptr].type_ = BlockType::Occupied;
        blocks[free_ptr].id = blocks[file_ptr].id;

        if blocks[file_ptr].occupied > blocks[free_ptr].free {
            let transfer =
                blocks[file_ptr].occupied - (blocks[file_ptr].occupied - blocks[free_ptr].free);
            blocks[file_ptr].occupied -= transfer;
            blocks[file_ptr].free += transfer;
            blocks[free_ptr].free -= transfer;
            blocks[free_ptr].occupied += transfer;
        } else {
            let transfer = blocks[file_ptr].occupied;
            blocks[file_ptr].free += transfer;
            blocks[file_ptr].occupied = 0;
            blocks[free_ptr].free -= transfer;
            blocks[free_ptr].occupied += transfer;

            if blocks[file_ptr].occupied == 0 {
                blocks[file_ptr].type_ = BlockType::Free;
                blocks[file_ptr].id = None;
            }

            if blocks[free_ptr].free > 0 {
                let newblock = Block::new_free(
                    blocks[free_ptr].free,
                    blocks[free_ptr].addr + blocks[free_ptr].occupied,
                );
                blocks[free_ptr].free = 0;
                blocks.insert(free_ptr + 1, newblock);
            }
        }
    }
}

fn get_block_by_id(blocks: &[Block], id: usize) -> usize {
    for (ptr, block) in blocks.iter().enumerate() {
        if let BlockType::Occupied = block.type_ {
            if block.id.unwrap() == id {
                return ptr;
            }
        }
    }
    unreachable!()
}

fn get_free_block_with_size(blocks: &[Block], size: usize) -> Option<usize> {
    for (ptr, block) in blocks.iter().enumerate() {
        if let BlockType::Free = block.type_ {
            if block.free >= size {
                return Some(ptr);
            }
        }
    }
    None
}

fn defragment(blocks: &mut Vec<Block>) {
    let mut curr_file_id = blocks
        .iter()
        .filter(|b| b.type_ == BlockType::Occupied)
        .map(|b| b.id.unwrap())
        .max()
        .unwrap();

    loop {
        if curr_file_id == 0 {
            break;
        }

        let file_ptr = get_block_by_id(blocks, curr_file_id);
        if let Some(free_ptr) = get_free_block_with_size(blocks, blocks[file_ptr].occupied) {
            if free_ptr > file_ptr {
                curr_file_id -= 1;
                continue;
            }

            blocks[free_ptr].type_ = BlockType::Occupied;
            blocks[free_ptr].id = blocks[file_ptr].id;

            let transfer = blocks[file_ptr].occupied;
            blocks[file_ptr].free += transfer;
            blocks[file_ptr].occupied = 0;
            blocks[free_ptr].free -= transfer;
            blocks[free_ptr].occupied += transfer;

            if blocks[file_ptr].occupied == 0 {
                blocks[file_ptr].type_ = BlockType::Free;
                blocks[file_ptr].id = None;
            }

            if blocks[free_ptr].free > 0 {
                let newblock = Block::new_free(
                    blocks[free_ptr].free,
                    blocks[free_ptr].addr + blocks[free_ptr].occupied,
                );
                blocks[free_ptr].free = 0;
                blocks.insert(free_ptr + 1, newblock);
            }
        }

        curr_file_id -= 1;
    }
}

fn main() {
    let data = include_str!("../input");

    let mut blocks_a = build_blocks(data);
    fragment(&mut blocks_a);
    let part_1 = blocks_a.iter().map(|b| b.checksum()).sum::<usize>();
    println!("Part 1: {part_1}");

    let mut blocks_b = build_blocks(data);
    defragment(&mut blocks_b);
    let part_2 = blocks_b.iter().map(|b| b.checksum()).sum::<usize>();
    println!("Part 2: {part_2}");
}
