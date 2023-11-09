use std::collections::HashMap;
use std::io::{Cursor, Error as IoError, Read};
use byteorder::{ReadBytesExt, BigEndian};

const REG_A: u8 = 1;
const REG_B: u8 = 2;
const REG_C: u8 = 3;
const REG_D: u8 = 4;
const REG_E: u8 = 5;

const INST_NOOP: u8 = 0;
const INST_LOAD_CONSTANT: u8 = 1;
const INST_ADD: u8 = 2;
const INST_SUBTRACT: u8 = 3;
const INST_COPY: u8 = 4;
const INST_COMPARE: u8 = 5;
const INST_JUMP: u8 = 6;
const INST_JUMP_IF_NON_ZERO: u8 = 7;
const INST_PRINT: u8 = 8;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args[1].trim();

    let bytes = std::fs::read(file_name).expect("Failed to read file.");

    let mut emulator = Emulator::new(bytes, false);
    emulator.run().unwrap();
}

fn gen_file() {

//  0: A = 0
//  1: B = 1
//  2: C = A + B
//  3: print(C)
//  4: A = B
//  5: B = C
//  6: D = 65536        }
//  7: D = C compare D  } C < 100
//  8: E = 1            }
//  9: E = D compare E  }
// 10: jump if E != 0 (if E is "true")

    let bytes = vec![
        INST_LOAD_CONSTANT,     REG_A, 0, 0, 0, 0,
        INST_LOAD_CONSTANT,     REG_B, 0, 0, 0, 1,
        INST_ADD,               REG_A, REG_B, REG_C,
        INST_PRINT,             REG_C,
        INST_COPY,              REG_B, REG_A,
        INST_COPY,              REG_C, REG_B,
        INST_LOAD_CONSTANT,     REG_D, 0, 1, 0, 0,
        INST_COMPARE,           REG_C, REG_D, REG_D,
        INST_LOAD_CONSTANT,     REG_E, 0, 0, 0, 1,
        INST_COMPARE,           REG_D, REG_E, REG_E,
        INST_JUMP_IF_NON_ZERO,  REG_E, 0, 0, 0, 12
    ];

    std::fs::write("fibonacci.bin", bytes).unwrap();

}

#[derive(Debug)]
pub enum EmulatorError {
    FailedToRead(IoError),
    InvalidInstruction(u8),
    InvalidRegistry(u8),
}

impl From<IoError> for EmulatorError {
    fn from(err: IoError) -> Self {
        EmulatorError::FailedToRead(err)
    }
}

struct Emulator {
    bytes: Cursor<Vec<u8>>,
    registries: HashMap<u8, i32>,
    instruction_pointer: usize,
    debug: bool,
}

impl Emulator {
    pub fn new(bytes: Vec<u8>, debug: bool) -> Self {
        Self {
            bytes: Cursor::new(bytes),
            registries: HashMap::new(),
            instruction_pointer: 0,
            debug
        }
    }

    pub fn run(&mut self) -> Result<(), EmulatorError> {
        loop {
            if self.bytes.position() >= self.bytes.get_ref().len() as u64 {
                break;
            }
            let inst = self.bytes.read_u8()?;
            if self.debug { println!("inst = {inst}"); }
            match inst {
                INST_NOOP => {
                    // no operation, do nothing
                },
                INST_LOAD_CONSTANT => {
                    let reg = self.read_registry()?;
                    if self.debug { println!("reg = {reg:?}"); }
                    let constant = self.bytes.read_i32::<BigEndian>()?;
                    if self.debug { println!("constant = {constant}"); }

                    self.registry_set(reg, constant);
                }
                INST_ADD => {
                    let a = self.read_registry()?;
                    let b = self.read_registry()?;
                    let result = self.read_registry()?;

                    let a_val = self.registry_get(a);
                    let b_val = self.registry_get(b);
                    let result_val = a_val + b_val;
                    self.registry_set(result, result_val);
                }
                INST_SUBTRACT => {
                    let a = self.read_registry()?;
                    let b = self.read_registry()?;
                    let result = self.read_registry()?;

                    let a_val = self.registry_get(a);
                    let b_val = self.registry_get(b);
                    let result_val = a_val - b_val;
                    self.registry_set(result, result_val);
                }
                INST_COPY => {
                    let from = self.read_registry()?;
                    let to = self.read_registry()?;

                    let val = self.registry_get(from);
                    self.registry_set(to, val);
                }
                INST_COMPARE => {
                    let a = self.read_registry()?;
                    let b = self.read_registry()?;
                    let result = self.read_registry()?;

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
                }
                INST_JUMP => {
                    let addr = self.read_address()?;
                    self.jump(addr);
                }
                INST_JUMP_IF_NON_ZERO => {
                    let reg = self.read_registry()?;
                    let addr = self.read_address()?;

                    let val = self.registry_get(reg);
                    if val != 0 {
                        self.jump(addr);
                    }
                }
                INST_PRINT => {
                    let reg = self.read_registry()?;

                    let val = self.registry_get(reg);
                    println!("{}", val);
                }
                _ => return Err(EmulatorError::InvalidInstruction(inst)),
            };
        }
        Ok(())
    }

    fn read_registry(&mut self) -> Result<u8, EmulatorError> {
        Ok(self.bytes.read_u8()?)
    }

    fn read_address(&mut self) -> Result<u32, EmulatorError> {
        Ok(self.bytes.read_u32::<BigEndian>()?)
    }

    fn jump(&mut self, to: u32) {
        self.bytes.set_position(to as u64);
    }

    fn registry_get(&self, registry: u8) -> i32 {
        self.registries.get(&registry).unwrap_or(&0).clone()
    }

    fn registry_set(&mut self, registry: u8, val: i32) {
        self.registries.insert(registry, val);
    }
}