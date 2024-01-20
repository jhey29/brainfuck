# writing an assembly language in bf take 2
thing I want to do: 
- disassemble each instruction into its bits and then check each one. 
  Propably requires about 9 to 10 bits around the instruction pointer or maybe an off-site register.
- make it stack-based, except maybe for some quick registers
- or make it so stack based that even the functions live on the stack
- make it aligned such that boundary markers only occur on cells divisible by e.g. 10



# Memory Layout
The Memory Layout repeats every 10 bytes. it is made 8 data bytes, and two bytes for e.g. a carry flag and a separator. 
S76543210F S76543210F S76543210F S76543210F 
The first Separator is always -1.

