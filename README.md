`cargo run -- fibonacci.bin`

| Instruction        | Byte value (u8) | Operands                                               |
|--------------------|-----------------|--------------------------------------------------------|
| `NOOP`             | 0               |                                                        |
| `LOAD_CONSTANT`    | 1               | registry (u8), constant (i32)                          |
| `ADD`              | 2               | registry a (u8), registry b (u8), result registry (u8) |
| `SUBTRACT`         | 3               | registry a (u8), registry b (u8), result registry (u8) |
| `COPY`             | 4               | from registry (u8), to registry (u8)                   |
| `COMPARE`          | 5               | registry a (u8), registry b (u8), result registry (u8) |
| `JUMP`             | 6               | to address (u32)                                       |
| `JUMP_IF_NON_ZERO` | 7               | registry (u8), to address (u32)                        |
| `PRINT`            | 8               | registry (u8)                                          |