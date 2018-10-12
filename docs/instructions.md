# Instructions

## Load instructions

| Instruction | Parameters | Description                        | Execution time | Opcode      |
| ----------- | ---------- | ---------------------------------- | -------------- | ----------- |
| ld          | X, Y       | load from register Y to register X |                | 01 xy 00 00 |
| ld          | X, A       | load from address A to register X  |                | 01 0x aa aa |
| ld          | X, V       | load value V to register X         |                | 01 x0 vv 00 |

## Arithmetical instructions

**Adding:**

| Instruction | Parameters | Description                                               | Opcode      |
| ----------- | ---------- | --------------------------------------------------------- | ----------- |
| add         | X, Y       | add register Y to register X                              | 02 00 xy 00 |
| adc         | X, Y       | add register Y plus the carry flag (0 or 1) to register X | 02 01 xy 00 |

**Subtracting:**

| Instruction | Parameters | Description                                                      | Opcode      |
| ----------- | ---------- | ---------------------------------------------------------------- | ----------- |
| sub         | X, Y       | subtract register Y from register X                              | 02 10 xy 00 |
| sbc         | X, Y       | subtract register Y plus the carry flag (0 or 1) from register X | 02 11 xy 00 |

**Increasing/Decreasing:**

| Instruction | Parameters | Description                | Opcode      |
| ----------- | ---------- | -------------------------- | ----------- |
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

## Jump instructions

| Instruction | Parameters | Description                              | Opcode      |
| ----------- | ---------- | ---------------------------------------- | ----------- |
| jmp         | A          | unconditional jump to address A          | 05 00 aa aa |    
| jmp         | C, A       | jump to address A if condition C is true | see below   |

The list below shows all possible conditions and the corresponding *jmp* **test** *test* command opcodes:

| Condition | Description                          | Opcode      |
| --------- | ------------------------------------ | ----------- |
| C         | Jump if carry flag (C) is set        | 05 01 aa aa |
| NC        | Jump if carry flag (C) is not set    | 05 0A aa aa |
| Z         | Jump if zero flag (Z) is set         | 05 02 aa aa |
| NZ        | Jump if zero flag (Z) is not set     | 05 0B aa aa |
| NEG       | Jump if negative flag (N) is set     | 05 03 aa aa |
| POS       | Jump if negative flag (N) is not set | 05 0C aa aa |

## Subroutine instructions

| Instruction | Parameters | Description                                                            | Opcode      |
| ----------- | ---------- | ---------------------------------------------------------------------- | ----------- |
| call        |            | pushes the address of the following instruction to the stack and jumps |             |
| ret         |            | pops an address from the top of the stack and jumps there              |             |

## Debug instructions

| Instruction | Parameters | Description                           | Opcode      |
| ----------- | ---------- | ------------------------------------- | ----------- |
| dbg         | X          | debug (print data, only in simulator) | 0A 0x 00 00 |
| halt        |            | halt CPU                              | 0A 00 00 00 |
| nop         |            | no operation                          | 0A FF FF FF |