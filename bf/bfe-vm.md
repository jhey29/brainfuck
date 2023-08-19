# writing an assembly language in brainfuck

## Memory Layout
The memory is first set up like this:
-1 00000000 00000000 00000000 00000000 -1 0  0000000 -1 0 23 
   C0 .. C7 B0 .. B7 A0 .. A7 S0 .. S7 dp Sc E1 ..E7    pp
At each -1, Memory alternates between 
data region before the data pointer,
data region after the data pointer,
and function region. 
### The data region
The first set-up data region is 41 in length, but data regions may be any bigger.
seen from the right, the first 8 bytes are counters, of which the eighth one is most used, which is why I called it Sc.
Then comes the data pointer parking space, 
and then all the registers, which smoothly transition into other memory depending on which is the last one I hard-code a use in for.
This means accessing a data region using a set offset from the left doesn't make sense.
### The Function region
Except before being initialized or in the execution of ! during setup, a Function region always contains exactly one 0 with either a -1 or another 0 to its left, 
and contains no other 0s or -1s. The case with the -1 on its left is only shortly during the giant switch statement's functioning, otherwise there are 2 0s.
This makes the one on the left the run flag, and the
## Instruction set
The instructions are not designed to have operands, because copying data in this variable way is hard to do in brainfuck.
They are all one character.
SPACE: no-op.
! : Write user function to the Sc-th next function region to the right and run it.
   This is questionably safe if the function region written to is not the last one, because 
   If 
/ : Set the S-register to 0.
01234567 : Set bit x of the function's S-register, regardless of whether it has been set already.

Bonus Idea: addition of Sc into E1 to E7 based on the bit pattern in S!