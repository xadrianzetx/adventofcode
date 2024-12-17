#[derive(Default)]
struct Interpreter {
    // General purpose registers.
    ra: usize,
    rb: usize,
    rc: usize,
    // Program counter.
    pc: usize,
    // Two extra registers to hold program output and its magnitude.
    rd: usize,
    re: u32,
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

macro_rules! rdiv {
    ($fun:ident, $register:ident) => {
        fn $fun(&mut self, operand: usize) {
            self.$register = self.ra / 2_usize.pow(operand as u32);
            self.advance();
        }
    };
}

impl Interpreter {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            ra: a,
            rb: b,
            rc: c,
            ..Default::default()
        }
    }

    fn run_program(&mut self, program: &[usize]) {
        loop {
            if self.pc >= program.len() {
                break;
            }

            let instruction = self.decode_instruction(program);
            self.execute_instruction(instruction);
        }
    }

    fn decode_instruction(&mut self, program: &[usize]) -> Instruction {
        let opcode = program[self.pc];
        let operand = program[self.pc + 1];
        Instruction::from_opcode_with_operand(opcode, operand)
    }

    fn decode_combo_operand(&self, operand: usize) -> usize {
        if operand <= 3 {
            return operand;
        }

        match operand {
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!(),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            Adv(operand) => self.adv(self.decode_combo_operand(operand)),
            Bxl(operand) => self.bxl(operand),
            Bst(operand) => self.bst(self.decode_combo_operand(operand)),
            Jnz(operand) => self.jnz(operand),
            Bxc => self.bxc(),
            Out(operand) => self.out(self.decode_combo_operand(operand)),
            Bdv(operand) => self.bdv(self.decode_combo_operand(operand)),
            Cdv(operand) => self.cdv(self.decode_combo_operand(operand)),
        };
    }

    fn advance(&mut self) {
        self.pc += 2;
    }

    rdiv!(adv, ra);
    rdiv!(bdv, rb);
    rdiv!(cdv, rc);

    fn bxl(&mut self, operand: usize) {
        self.rb ^= operand;
        self.advance();
    }

    fn bst(&mut self, operand: usize) {
        self.rb = operand % 8;
        self.advance();
    }

    fn jnz(&mut self, operand: usize) {
        if self.ra == 0 {
            self.advance();
        } else {
            self.pc = operand;
        }
    }

    fn bxc(&mut self) {
        self.rb ^= self.rc;
        self.advance();
    }

    fn out(&mut self, operand: usize) {
        self.rd += (operand % 8) * (10_usize.pow(self.re));
        self.re += 1;
        self.advance();
    }

    fn flush(&self) -> String {
        self.rd
            .to_string()
            .chars()
            .rev()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    #[inline]
    fn iflush(&self) -> usize {
        self.rd
    }
}

fn main() {
    let program = [2, 4, 1, 1, 7, 5, 4, 7, 1, 4, 0, 3, 5, 5, 3, 0];
    let mut interpreter = Interpreter::new(30553366, 0, 0);
    interpreter.run_program(&program);
    let part_1 = interpreter.flush();
    println!("Part 1: {part_1}");

    // My Chronospatial Computer intcode decompiles to following pseudocode:
    //
    //loop {
    // 	b = a % 8;      // b is set to 0..=7
    // 	b = b ^ 1;      // lowest b bit is flipped
    // 	c = a / 2**b;   // a divided by 2**(0..=7)
    // 	b = b ^ c;      // b xored with some number dependent on a
    // 	b = b ^ 4;      // third bit from right in b is flipped
    // 	a = a / 8;      // div a by 8 to go to next iter
    // 	print(b % 8);   // b truncated to range 0..=7 and printed
    // 	if a == 0 {
    //    break;
    // 	}
    //}
    //
    // This basically means that we can try matching the program digit by digit (starting from the last one)
    // by just multiplying previous correct reg a state by 8 and then searching a small region for next valid
    // reg a state (i.e. one that prints out correct code up to that point). Neat.
    let mut part_2: usize = 0;
    let program_len = program.len();
    for mask in (0..program_len).rev() {
        let expected_out = &program[mask..]
            .iter()
            .enumerate()
            .map(|(pow, num)| num * 10_usize.pow(pow as u32))
            .sum();

        for offset in 0..=500 {
            let mut interpreter = Interpreter::new(part_2 * 8 + offset, 0, 0);
            interpreter.run_program(&program);
            if &interpreter.iflush() == expected_out {
                part_2 = part_2 * 8 + offset;
                break;
            }
        }
    }
    println!("Part 2: {part_2}");
}
