const STACK_SIZE: usize = 1024;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum OpCode {
    PUSH(i32),
    ADD,
    CMP, // 1 if equals, 0 if different
    JMP(usize), // jump if top is 1
    PRINT,
    HALT
}

pub struct Instruction {
    label: usize,
    opcode: OpCode
}

impl Instruction {
    pub fn new(label: usize, opcode: OpCode) -> Self {
        Self { label, opcode }
    }
}

pub struct VirtualMachine {
    program: Vec<Instruction>,
    stack: Vec<i32>,
    pc: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            program: vec![],
            stack: vec![0; STACK_SIZE],
            pc: 0
        }
    }
    
    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    pub fn execute(&mut self) {
        let mut is_running = true;
        while is_running {
            let Instruction { opcode, .. } = &self.program[self.pc];
            match opcode {
                OpCode::PUSH(val) => {
                    self.stack.push(*val);
                },
                OpCode::ADD => {
                    let (a, b) = (self.stack.pop(), self.stack.pop());
                    if a.is_some() && b.is_some() {

                    } else {
                        println!("[ERROR] Not enough data in the stack.");
                    }
                },
                OpCode::CMP => is_running = false,
                OpCode::JMP(loc) => is_running = false,
                OpCode::PRINT => {
                    if let Some(data) = self.stack.pop() {
                        println!("> {}", data);
                    }
                },
                OpCode::HALT => is_running = false
            }
        }
    }
}

// The VM is gonna execute the following program:
//
//     a = 10                  
//     b = 20                  
//     c = a + b               
//     if c == 30 {             
//         print(1)            
//     } else {
//         print(2)
//     }
//
fn main() {
    let mut vm = VirtualMachine::new();
    vm.load_program(vec![
        Instruction::new(1, OpCode::PUSH(10)),          // a = 10
        Instruction::new(2, OpCode::PUSH(20)),          // b = 20
        Instruction::new(3, OpCode::ADD),               // c = a + b
        Instruction::new(4, OpCode::PUSH(30)),          
        Instruction::new(5, OpCode::CMP),               // if c == 30
        Instruction::new(6, OpCode::JMP(10)),           
        Instruction::new(7, OpCode::PUSH(1)),           
        Instruction::new(8, OpCode::PRINT),             // print(1)
        Instruction::new(9, OpCode::JMP(13)),
        Instruction::new(10, OpCode::PUSH(2)),
        Instruction::new(11, OpCode::PRINT),            // else print(2)
        Instruction::new(12, OpCode::JMP(13)),
        Instruction::new(13, OpCode::HALT),
    ]);
    vm.execute();
    println!("{:#?}", vm.stack);
}