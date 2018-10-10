# Instructions

## Load instructions

| Instruction | Parameters | Description                        | Opcode      |
| ----------- | ---------- | ---------------------------------- | ----------- |
| ld          | X, Y       | load from register Y to register X | 01 xy 00 00 |
| ld          | X, A       | load from address A to register X  | 01 0x aa aa |
| ld          | X, V       | load value V to register X         | 01 x0 vv 00 |

## Arithmetical instructions

**Adding:**

| Instruction | Parameters | Description                  | Opcode      |
| ----------- | ---------- | ---------------------------- | ----------- |
| add         | X, Y       | add register Y to register X | 02 00 xy 00 |
| adc         | X, Y       |                              | 02 01 xy 00 |

**Subtracting:**

| Instruction | Parameters | Description                         | Opcode      |
| ----------- | ---------- | ----------------------------------- | ----------- |
| sub         | X, Y       | subtract register Y from register X | 03 02 xy 00 |
| sbc         | X, Y       |                                     | 03 03 xy 00 |

**Increasing/Decreasing:**

| Instruction | Parameters | Description                | Opcode      |
| ----------- | ---------- | --------------------- ---- | ----------- |
| inc         | X          | increase register X by one | 01 00 0x 00 |
| dec         | X          | decrease register X by one | 01 00 x0 00 |

## Bitwise instructions

| Instruction | Parameters | Description | Opcode      |
| ----------- | ---------- | ----------- | ----------- |
| and         | X, Y       |             | 04 00 xy 00 |
| or          | X, Y       |             | 04 01 xy 00 |
| xor         | X, Y       |             | 04 02 xy 00 |
| not         | X          |             | 04 03 0x 00 |

## Stack instructions

| Instruction | Parameters | Description | Opcode      |
| ----------- | ---------- | ----------- | ----------- |
| push        |            |             |             |
| pop         |            |             |             |

## Compare and jump instructions

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

## Debug instructions

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