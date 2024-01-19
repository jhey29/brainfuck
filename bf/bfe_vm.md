# writing an assembly language in bf take 2
thing I want to do: 
- disassemble each instruction into its bits and then check each one. 
  Propably requires about 9 to 10 bits around the instruction pointer or maybe an off-site register.
- make it stack-based, except maybe for some quick registers
- make it aligned such that boundary markers only occur on cells divisible by e.g. 10
