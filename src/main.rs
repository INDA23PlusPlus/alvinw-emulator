use std::collections::HashMap;

fn main() {
    let mut registries: HashMap<Registry, i32> = HashMap::new();

    let inst = [
        Instruction::LoadConstant(Registry::A, 1),
        Instruction::LoadConstant(Registry::B, 2),
        Instruction::Add(Registry::A, Registry::B, Registry::C),
        Instruction::Print(Registry::C),
    ];

    for inst in inst {
        match inst {
            Instruction::LoadConstant(reg, val) => {
                registries.insert(reg, val);
            },
            Instruction::Add(a, b, result) => {
                let a_val = registries.get(&a).unwrap_or(&0);
                let b_val = registries.get(&b).unwrap_or(&0);
                let result_val = a_val + b_val;
                registries.insert(result, result_val);
            },
            Instruction::Print(reg) => {
                let val = registries.get(&reg).unwrap_or(&0);
                println!("{}", val);
            },
        }
    }
}


#[derive(Hash, PartialEq, Eq)]
enum Registry {
    Stack,
    A,
    B,
    C,
}

enum Instruction {
    LoadConstant(Registry, i32),
    Add(Registry, Registry, Registry),
    Print(Registry),
}