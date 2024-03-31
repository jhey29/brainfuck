# BrainPipe 
BrainPipe is a Brainfuck variant that is meant to separate the fun part
of writing brainfuck programs, which is writing novel turing-machine-esque
solutions for a task, from the boring and frustrating part, which is 
large-scale memory management and input handling. 

It currently adds one feature: pipes.
Pipes can be placed between two or more functionally independent 
(with closed brackets) brainfuck programs, whose inputs and outputs
are then chained together. The first is connected to stdin and the last
is connected to stdout. The last program is run until it requires input,
at which point the program above it will run until it has provided this input,
or has requested input itself. If any of these brainfuck programs ends, the entire thing will. 
