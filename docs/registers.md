# Registers

There may not be any registers with the number 0000 (0) or 1111 (F) as it would cause trouble with the opcode interpretation.

## Single 8-bit registers

| Register | Description                                              | Bin  | Hex |
| -------- | -------------------------------------------------------- | ---- | --- |
| pc1      | One of the two registers used for the 16-bit pc register | 0001 | 1   |
| pc2      | One of the two registers used for the 16-bit pc register | 0010 | 2   |
| sp1      | One of the two registers used for the 16-bit sp register | 0011 | 3   |
| sp2      | One of the two registers used for the 16-bit sp register | 0100 | 4   |
| a        | General purpose register                                 | 0101 | 5   |
| b        | General purpose register                                 | 0110 | 6   |
| c        | General purpose register                                 | 0111 | 7   |
| d        | General purpose register                                 | 1000 | 8   |
| e        | General purpose register                                 | 1001 | 9   |
| f        | General purpose register                                 | 1010 | A   |
| g        | General purpose register                                 | 1011 | B   |
| h        | General purpose register                                 | 1100 | C   |

## Double 16-bit registers

16-bit registers consist of two 8-bit registers that are used to store a 16-bit value.
Keep in mind that usage of one of the two 8-bit registers results in corrupting a
possibly stored 16-bit value.

| Register | Description                                              | Bin  | Hex |
| -------- | -------------------------------------------------------- | ---- | --- |
| pc       | Program counter                                          | 0001 | 1   |
| sp       | Contains the address of the topmost element on the stack | 0011 | 3   |
| cd       | General purpose register                                 | 0111 | 7   |
| ef       | General purpose register                                 | 1001 | 9   |