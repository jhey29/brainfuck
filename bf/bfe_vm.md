# writing an assembly language in brainfuck

(hybris 5000)

## Memory Layout
The memory is first set up like this:
-1 00000000 00000000 00000000 00000000 -1  0 0000000 -1 0   0   23 
   C0 .. C7 B0 .. B7 A0 .. A7 S0 .. S7 dp Sc E1 ..E7    run ppp 
At each -1, Memory alternates between 
data region before the data pointer,
data region after the data pointer,
and function region. 

### The data region
The first set-up data region is 41 in length, but data regions may be any bigger.
seen from the right, the first 8 bytes are counters, of which the eighth one is most used, which is why I called it Sc.
Then comes the data pointer (dp) parking space, and then all the memory propably intended for bitwise bytes, 
which are technically wasteful but way easier and faster to do maths on. This does not matter anyways.
The region to the left of the dp is accessed by instructions using offsets relative to the dp. 
Normally, they are intended to be 8-bit bytes 
The region to the right of the dp is not completely destroyed when the dp moves over it, but the bit where the dp currently resides is
relocated to the dp parking space to allow the dp to sit anywhere to the left of it. That part should not be read anyways. 


### The Function region
Except before being initialized, and during the first part of the giant switch statement,
a Function region always contains exactly one 0 with either a -1 or another 0 to its left, and contains no other 0s or -1s. 

The one on the left is the run flag and the one on the right is the persistent pretty picture.
The giant switch statement that contains the code of all instructions has a repeated structure for each:
it opens a loop for each instruction, slowly decreasing the instruction numerically, it reaches zero.
Then, it inc's the run flag to 1 to enter the instructions code container, and dec's it to zero inside.
The stuff that is done is expected to park the bfMPTR on the run flag (also zero), to exit the loop that it is contained in. Then the structural
boilerplate takes over and Sets the run flag to -1, using the ppp to fall through the next nested loop of the switch statement.
If the run flag is -1, all the further cases of the giant switch statement don't run. The far bottom of the main loop then sets it up to read the next instruction. 


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