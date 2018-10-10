# Instructions

## Load instructions

| Instruction | Parameters |
| ----------- | ---------- |
| ld          | A, B       |

**A can be:**

- Register

**B can be:**

- Value: $1234
- Register
- Address
- Label

**Opcodes:**

| Opcode      | Description                        |
| ----------- | ---------------------------------- |
| 01 xy 00 00 | load from register Y to register X |
| 01 0r aa aa | load from address to register      |
| 01 r0 vv 00 | load value to register             |

### Arithmetical instructions

**Adding:**

| Instruction | Parameters | Opcode      |
| ----------- | ---------- | ----------- |
| add         | R, R       | 02 00 rr 00 |
| adc         | R, R       | 02 01 rr 00 |

**Subtracting:**

| Instruction | Parameters | Opcode      |
| ----------- | ---------- | ----------- |
| sub         | R, R       | 03 02 rr 00 |
| sbc         | R, R       | 03 03 rr 00 |

**Increasing/Decreasing:**

| Instruction | Parameters | Opcode      |
| ----------- | ---------- | ----------- |
| inc         | R          | 01 00 0r 00 |
| dec         | R          | 01 00 r0 00 |

### Bitwise instructions

| Instruction | Parameters | Opcode      |
| ----------- | ---------- | ----------- |
| and         |            |             |
| or          |            |             |
| xor         |            |             |
| not         |            |             |

### Stack instructions

| Instruction | Parameters | Opcode      |
| ----------- | ---------- | ----------- |
| push        |            |             |
| pop         |            |             |

### Compare and jump instructions

| Instruction | Parameters | Description | Opcode      |
| ----------- | ---------- | ----------- | ----------- |
| cmp         |            | Compare     |             |

| Instruction | Parameters | Description        | Opcode      |
| ----------- | ---------- | ------------------ | ----------- |
| jmp         |            | unconditional jump |             |    
| jgr         |            | jump if greater    |             |
| jle         |            | jump if less       |             |
| jz          |            | jump if zero       |             |
| jnz         |            | jump if not zero   |             |

| Instruction | Parameters | Description                                                            | Opcode      |
| ----------- | ---------- | ---------------------------------------------------------------------- | ----------- |
| call        |            | pushes the address of the following instruction to the stack and jumps |             |
| ret         |            | pops an address from the top of the stack and jumps there              |             |

### Debug instructions

| Instruction | Parameters | Description                           | Opcode      |
| ----------- | ---------- | ------------------------------------- | ----------- |
| dbg         | R          | debug (print data, only in simulator) | 0A 0r 00 00 |
| halt        |            | halt CPU                              | 0A 00 00 00 |
| nop         |            | no operation                          |             |


## Flags

| Flag | Description                                  |
| ---- | -------------------------------------------- |
| S    | set if result (two's complement) is negative |
| Z    | set if result is zero                        |
| C    | set if result did not fit in the register    |


## Registers

There may not be any registers with the names 0000 (0) and 1111 (F).

| Register | Description     | Bin  | Hex |
| -------- | --------------- | ---- | --- |
| pc       | program counter | 0001 | 1   |
| sp       | stack pointer   | 0010 | 2   |
| a        |                 | 0011 | 3   |
| b        |                 | 0100 | 4   |
| c        |                 | 0101 | 5   |
| d        |                 | 0110 | 6   |
| e        |                 | 0111 | 7   |
| f        |                 | 1000 | 8   |
| g        |                 | 1001 | 9   |