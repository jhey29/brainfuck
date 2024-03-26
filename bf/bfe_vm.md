# writing an assembly language in bf take 2
thing I want to do: 
- disassemble each instruction into its bits and then check each one. 
  Propably requires about 9 to 10 bits around the instruction pointer. This is actually such a good idea.
- make it stack-based? on the macro level vvvvvvvvv
- or make it so stack based that even the functions live on the stack
- make it aligned such that boundary markers only occur on cells divisible by e.g. 10



# Memory Layout

The Scratch Space is
IN0JABCDEFGHiiiiii
N: -1
I,J: where the opcode most often is
A...H : generic bit space
i: next instruction (N >*11)

it can't carry over to the next instruction.
- the binary tree should contain a branch where every instruction reads, writes, etc.