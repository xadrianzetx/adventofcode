#[derive(Debug)]
struct Cpu {
    a: usize,
    b: usize,
    c: usize,
    ptr: usize,
}

enum Instruction {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc(usize),
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
            4 => Self::Bxc(operand),
            5 => Self::Out(operand),
            6 => Self::Bdv(operand),
            7 => Self::Cdv(operand),
            _ => unreachable!(),
        }
    }
}

impl Cpu {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c, ptr: 0 }
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
                self.a = self.a / 2_usize.pow(value as u32);
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
            Bxc(_) => {
                self.b ^= self.c;
                self.inc_ptr();
            }
            Out(operand) => {
                let value = self.decode_combo_operand(operand);
                print!("{},", value % 8);
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
}

fn main() {
    let program = [2, 4, 1, 1, 7, 5, 4, 7, 1, 4, 0, 3, 5, 5, 3, 0];
    let mut cpu = Cpu::new(30553366, 0, 0);
    cpu.run_program(&program);
    println!();
}
