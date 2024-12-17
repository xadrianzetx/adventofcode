#[derive(Debug)]
struct Cpu {
    a: usize,
    b: usize,
    c: usize,
    ptr: usize,
    charbuff: Vec<String>,
}

enum Instruction {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc,
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

impl Instruction {
    fn from_opcode_with_operand(opcode: usize, operand: usize) -> Self {
        match opcode {
            0 => Self::Adv(operand),
            1 => Self::Bxl(operand),
            2 => Self::Bst(operand),
            3 => Self::Jnz(operand),
            4 => Self::Bxc,
            5 => Self::Out(operand),
            6 => Self::Bdv(operand),
            7 => Self::Cdv(operand),
            _ => unreachable!(),
        }
    }
}

impl Cpu {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            ptr: 0,
            charbuff: Vec::new(),
        }
    }

    fn run_program(&mut self, program: &[usize]) {
        loop {
            if self.ptr >= program.len() {
                break;
            }

            let instruction = self.decode_instruction(program);
            self.execute_instruction(instruction);
        }
    }

    fn decode_instruction(&mut self, program: &[usize]) -> Instruction {
        let opcode = program[self.ptr];
        let operand = program[self.ptr + 1];
        Instruction::from_opcode_with_operand(opcode, operand)
    }

    fn decode_combo_operand(&self, operand: usize) -> usize {
        if operand <= 3 {
            return operand;
        }

        match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn inc_ptr(&mut self) {
        self.ptr += 2;
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            Adv(operand) => {
                let value = self.decode_combo_operand(operand);
                self.a /= 2_usize.pow(value as u32);
                self.inc_ptr();
            }
            Bxl(operand) => {
                self.b ^= operand;
                self.inc_ptr();
            }
            Bst(operand) => {
                let value = self.decode_combo_operand(operand);
                self.b = value % 8;
                self.inc_ptr();
            }
            Jnz(operand) => {
                if self.a == 0 {
                    self.inc_ptr();
                } else {
                    self.ptr = operand;
                }
            }
            Bxc => {
                self.b ^= self.c;
                self.inc_ptr();
            }
            Out(operand) => {
                let value = self.decode_combo_operand(operand);
                self.charbuff.push((value % 8).to_string());
                self.inc_ptr();
            }
            Bdv(operand) => {
                let value = self.decode_combo_operand(operand);
                self.b = self.a / 2_usize.pow(value as u32);
                self.inc_ptr();
            }
            Cdv(operand) => {
                let value = self.decode_combo_operand(operand);
                self.c = self.a / 2_usize.pow(value as u32);
                self.inc_ptr();
            }
        };
    }

    fn flush(&self) -> String {
        self.charbuff.join(",")
    }

    fn flush_no_sep(&self) -> String {
        self.charbuff.join("")
    }
}

fn main() {
    let program = [2, 4, 1, 1, 7, 5, 4, 7, 1, 4, 0, 3, 5, 5, 3, 0];
    let mut cpu = Cpu::new(30553366, 0, 0);
    cpu.run_program(&program);
    let part_1 = cpu.flush();
    println!("Part 1: {part_1}");

    // My Chronospatial Computer "assembly" decompiles to following pseudocode:
    //
    // // loop {
    // 	b = a % 8; 		// b is set to 0..=7
    // 	b = b ^ 1; 		// lowest b bit is flipped
    // 	c = a / 2^b; 	// a divided by 2^(0..=7)
    // 	b = b ^ c; 		// b xored with some number dependent on a
    // 	b = b ^ 4; 		// third bit from right in b is flipped
    // 	a = a / 8; 		// div a by 8 to go to next iter
    // 	print(b % 8);   // b truncated to range 0..=7 and printed
    // 	if a == 0 {
    // 		break;
    // 	}
    // }
    //
    // This basically means that we can try matching the program digit by digit (starting from the last one)
    // by just multiplying previous correct reg a state by 8 and then searching a small region for next valid
    // reg a state (i.e. one that prints out correct code up to that point). Neat.
    let mut part_2: usize = 0;
    let program_len = program.len();
    for mask in (0..program_len).rev() {
        let expected_out = &program[mask..]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("");

        for offset in 0..=500 {
            let mut cpu = Cpu::new(part_2 * 8 + offset, 0, 0);
            cpu.run_program(&program);
            if &cpu.flush_no_sep() == expected_out {
                part_2 = part_2 * 8 + offset;
                break;
            }
        }
    }
    println!("Part 2: {part_2}");
}
