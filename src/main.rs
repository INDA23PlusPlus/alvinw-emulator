use std::collections::HashMap;

fn main() {
    let mut emulator = Emulator::new();
    emulator.run();
}


struct Emulator {
    registries: HashMap<Registry, i32>,
    instruction_pointer: usize,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            registries: HashMap::new(),
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self) {
//  0: A = 0
//  1: B = 1
//  2: C = A + B
//  3: print(C)
//  4: A = B
//  5: B = C
//  6: D = 100          }
//  7: D = C compare D  } C < 100
//  8: E = -1           }
//  9: E = D compare E  }
// 10: jump if E != 0 (if E is "true")

        let instructions = [
            /*  0 */ Instruction::LoadConstant(Registry::A, 0),
            /*  1 */ Instruction::LoadConstant(Registry::B, 1),
            /*  2 */ Instruction::Add(Registry::A, Registry::B, Registry::C),
            /*  3 */ Instruction::Print(Registry::C),
            /*  4 */ Instruction::Copy(Registry::B, Registry::A),
            /*  5 */ Instruction::Copy(Registry::C, Registry::B),
            /*  6 */ Instruction::LoadConstant(Registry::D, 100),
            /*  7 */ Instruction::Compare(Registry::C, Registry::D, Registry::D),
            /*  8 */ Instruction::LoadConstant(Registry::E, -1),
            /*  9 */ Instruction::Compare(Registry::D, Registry::E, Registry::E),
            /* 10 */ Instruction::JumpIfNonZero(Registry::E, 2),
        ];

        loop {
            if self.instruction_pointer >= instructions.len() {
                break;
            }
            let inst = instructions[self.instruction_pointer];
            match inst {
                Instruction::LoadConstant(reg, val) => {
                    self.registry_set(reg, val);
                },
                Instruction::Add(a, b, result) => {
                    let a_val = self.registry_get(a);
                    let b_val = self.registry_get(b);
                    let result_val = a_val + b_val;
                    self.registry_set(result, result_val);
                },
                Instruction::Subtract(a, b, result) => {
                    let a_val = self.registry_get(a);
                    let b_val = self.registry_get(b);
                    let result_val = a_val - b_val;
                    self.registry_set(result, result_val);
                },
                Instruction::Print(reg) => {
                    let val = self.registry_get(reg);
                    println!("{}", val);
                },
                Instruction::Jump(to) => {
                    self.jump(to);
                },
                Instruction::JumpIfNonZero(reg, to) => {
                    let val = self.registry_get(reg);
                    if val == 0 {
                        self.jump(to);
                    }
                },
                Instruction::Copy(from, to) => {
                    let val = self.registry_get(from);
                    self.registry_set(to, val);
                },
                Instruction::Compare(a, b, result) => {
                    let a_val = self.registry_get(a);
                    let b_val = self.registry_get(b);
                    let result_val = if a_val == b_val {
                        0
                    } else if a_val < b_val {
                        -1
                    } else {
                        1
                    };
                    self.registry_set(result, result_val);
                },
            }
            self.instruction_pointer += 1;
        }
    }

    fn jump(&mut self, to: u32) {
        self.instruction_pointer = to as usize - 1;
    }

    fn registry_get(&self, registry: Registry) -> i32 {
        self.registries.get(&registry).unwrap_or(&0).clone()
    }

    fn registry_set(&mut self, registry: Registry, val: i32) {
        self.registries.insert(registry, val);
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Registry {
    Stack,
    A,
    B,
    C,
    D,
    E,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    LoadConstant(Registry, i32),
    Add(Registry, Registry, Registry),
    Subtract(Registry, Registry, Registry),
    Print(Registry),
    Jump(u32),
    JumpIfNonZero(Registry, u32),
    Copy(Registry, Registry),
    Compare(Registry, Registry, Registry),
}