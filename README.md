# min-vm
## A spec for a miniature virtual machine

min-vm is a tiny virtual machine designed to be easy to implement and write programs for.
Reference implementations are included, but please try to write your own before seeing how others have done it!

### Hardware
The __Machine__ has 256 bytes of random access memory (RAM).
These are initialized to 0.

The __Machine__ has 8 general purpose registers named __R0__, __R1__, __R2__, __R3__, __R4__, __R5__, __R6__, and __R7__.
Each register holds 1 byte.  
These are initialized to 0.

The __Machine__ has an instruction pointer __IP__.
It is initialized to 0.

### Instructions

A single instruction (opcode _and_ operands) takes 5 bytes for every instruction.

Each byte in the instruction is distinct in its purpose.

```
                 Source              Dest
           +---------------+   +---------------+
 Opcode    | Type    Value |   | Type    Value |
+------+   +------+ +------+   +------+ +------+
|      |   |      | |      |   |      | |      |
oooooooo   tttttttt vvvvvvvv   tttttttt vvvvvvvv
```

The Opcode decides what instruction is being decoded.  These values are listed below.

The source and destination operands are arguments used by the instruction and are dependant on the operand.

## Operands 
Operands are each broken into two-byte chunks.  The first byte determines what the "type" the operand is.  

These _types_ are 
* 0: Constant
* 1: Register
* 2: Memory-Constant 
* 3: Memory-From-Register

When the _type_ is "constant", the number stored inside _value_ is used.

When the _type_ is "register", _value_ must be in the range 0-9 and a register is selected based on the number (0 chooses __R0__, 1 chooses __R1__, etc...).
If the __value__ is above 7, the VM should crash.

When the _type_ is "memory-constant", _value_ is used as a location in memory.

When thye _type_ is "memory-from-register", _value_ must be in the range 0-9 and a register is selected like above.  The value in the register is used to select a memory position.

### Operand Examples
* `0x00 5`: Literal 5
* `0x01 2`: Register 2
* `0x02 0x43`: Memory Location `0x43`
* `0x00 0x43`: Literal `0x43`

## Opcodes

num  | Opcode | Operand 1 | Operand 2 |Description
--- | ------------- | ---- | ---- | -------------
0 | CRASH  | | | If opcode 0 is executed, the VM should crash
1 | PRINT  | Source | | Converts Source to an ASCII char and prints it
2 | ADD    | Source | Dest | Computes (Dest + Source). Stores the result in Dest
3 | SUB    | Source | Dest | Computes (Dest - Source). Stores the result in Dest
4 | MUL    | Source | Dest | Computes (Dest * Source). Stores the result in Dest
5 | DIV    | Source | Dest | Computes (Dest * Source). Stores the result in Dest 
6 | MOD    | Source | Dest | Computes (Dest % Source). Stores the result in Dest 
7 | JMP    | Location | | Sets the __Instruction Pointer__ to Location
8 | CMP    | Source | Dest | Compares Source and Dest and writes into Dest: 0 if they are equal, 1 if Source > Dest, 255 if Source < Dest
9 | AND    | Source | Dest | Computes (Dest & Source). Stores the result in Dest
10| OR     | Source | Dest | Computes (Dest | Source). Stores the result in Dest
11| IF     | Condition | | If Condition is 0 the next operation is skipped
12| CALL   | Location | WriteNext | Sets the __Instruction Pointer__ to Location.  Writes the previous __Instruction Pointer__ to WriteNext.
13| MOVE   | Source | Dest | Sets Source to Dest

## Examples
```
1,  0, 5,  1 2  # Add the literal 5 and the value in __R2__.  Store the result in __R2__.
7,  2, 34       # Read the number out of memory location 34 and put the result in the instruction pointer.
4,  1, 3,  2 50 # Multiplies __R3__ and the value at memory location 50.  Stores the result in memory location 50.
8,  3, 0,  3 1  # Compares the values in memory pointed to by __R0__ and __R1__ and stores the result in the memory location stored in __R1__.
```
