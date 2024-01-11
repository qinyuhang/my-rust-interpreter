/// A simple VM
enum Instruction {
    PUSH,
    ADD,
    SUB,
    MUL,
    DIV,
}
struct VM {
    pub pc: usize,
    pub sp: usize,
    pub stack: [i32; 4096],
}
impl VM {
    pub fn default() -> Self {
        Self {
            pc: 0,
            sp: 0,
            stack: [0; 4096],
        }
    }

    pub fn run(mut self, program: Vec<(Instruction, Option<i32>)>) -> i32 {
        while self.pc < program.len() {
            let (current_instruction, data) = &program[self.pc];

            let mut left = 0i32;
            let mut right = 0i32;

            match current_instruction {
                Instruction::PUSH => {
                    self.stack[self.sp] = data.unwrap();
                    self.sp += 1;
                    // self.pc += 1;
                }
                Instruction::ADD => {
                    right = self.stack[self.sp - 1];
                    self.sp -= 1;
                    left = self.stack[self.sp - 1];
                    self.sp -= 1;
                    self.stack[self.sp] = left + right;
                    self.sp += 1;
                }
                Instruction::SUB => {
                    right = self.stack[self.sp - 1];
                    self.sp -= 1;
                    left = self.stack[self.sp - 1];
                    self.sp -= 1;
                    self.stack[self.sp] = left - right;
                    self.sp += 1;
                }
                Instruction::MUL => {
                    right = self.stack[self.sp - 1];
                    self.sp -= 1;
                    left = self.stack[self.sp - 1];
                    self.sp -= 1;
                    self.stack[self.sp] = left * right;
                    self.sp += 1;
                }
                Instruction::DIV => {
                    right = self.stack[self.sp - 1];
                    self.sp -= 1;
                    left = self.stack[self.sp - 1];
                    self.sp -= 1;
                    self.stack[self.sp] = left / right;
                    self.sp += 1;
                }
            }
            self.pc += 1;
        }
        self.stack[self.sp - 1]
    }
}

fn main() {
    println!("Hello, world!");
    let vm = VM::default();
    let program = vec![
        (Instruction::PUSH, Some(3)),
        (Instruction::PUSH, Some(4)),
        (Instruction::ADD, None),
        (Instruction::PUSH, Some(5)),
        (Instruction::SUB, None),
        (Instruction::PUSH, Some(10)),
        (Instruction::MUL, None),
        (Instruction::PUSH, Some(10)),
        (Instruction::DIV, None),
    ];
    println!("{}", vm.run(program));
    println!("Bye, world!");
}
