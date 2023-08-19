[ build the first data section (-1 at  address 0, 33, 42)  ] 
->+++++ +++++ [[>>>+<<<-]>>>-]>>>->>>>> >>>>- the data pointer neg1 at address 33 complicates this
then write the number 23 ( ! ) to the function buffer 
three spaces over >>>
>++++[-<++++++>]<-
then run the function 
idiomatically sitting on its progress 0
when an instruction finishes  it returns to this 0 
but we actually need to sit on the instruction otherwise the main loop doesnt run
main loop : #
[ 
giant switch statement mode! 
1-<<+>>[ 2-<<+>>[ 3-<<+>>[ 4-<<+>>[
22----- ----- ----- ---  <<+++++ +++++ +++++ +++  >>[
23-<<+>>[ 24-<<+>>[ 25-<<+>>[ 26-<<+>>[ 27-<<+>>[ 28-<<+>>[ 29-<<+>>[ 30-<<+>>[ 31-<<+>>[ 32-<<+>>[ 33-<<+>>[ 34-<<+>>[ 35-<<+>>[ 36-<<+>>[ 37-<<+>>[ 
38-<<+>>[ 39-<<+>>[ 40-<<+>>[ 41-<<+>>[ 42-<<+>>[ 43-<<+>>[ 44-<<+>>[ 45-<<+>>[ 46-<<+>>[
tbd!
for each instruction: we are sitting on the second 0 we just made anew except inside the brackets we are actually sitting on the first one (the run flag)
add one to the run flag to make self run   if it runs: {reduce the one again  do stuff   
return to the SECOND 0 in the function to break the 'loop'}    
then go to the run flag 
set the run flag to neg1 so the other instrs don't run  go back to the instr_hole zero to make the switch loop fall through
]46: <+ [- noop 8 [>] ]8 [+]->
]45: <+ [- noop 7 [>] ]7 [+]->
]44: <+ [- noop 6 [>] ]6 [+]->
]43: <+ [- noop 5 [>] ]5 [+]->
]42: <+ [- noop 4 [>] ]4 [+]->
]41: <+ [- noop 3 [>] ]3 [+]->
]40: <+ [- noop 2 [>] ]2 [+]->
]39: <+ [- noop 1 [>] ]1 [+]->
]38: <+ [- noop 0 [>] ]0 [+]->
]37: <+ [- noop / [>] ]/ [+]->
]36: <+ [- noop dot [>] ]dot [+]->
]35: <+ [- noop minus [>] ]minus [+]->
]34: <+ [- noop comma [>] ]comma [+]->
]33: <+ [- noop plus [>] ]plus [+]->
]32: <+ [- noop * [>] ]* [+]->
]31: <+ [- noop ) [>] ]) [+]->
]30: <+ [- noop ( [>] ]( [+]->
]29: <+ [- noop ' [>] ]' [+]->
]28: <+ [- noop & [>] ]& [+]->
]27: <+ [- noop % [>] ]% [+]->
]26: <+ [- noop $ [>] ]$ [+]->
]25: <+ [- noop # [>] ] [+]->
]24: <+ [- noop "# [>] ]" [+]->
]23: <+ [- The ! Instruction:  go to Sc  +[-<+]- <<<< <<<<
    Sc transport:repeat for offset [
    Sc transport:lift Sc one data pool over [  -  +[->+]- > +[->+]- > +[->+]- (dp) > +[->+]- <<<< <<<<  +  +[-<+]- < +[-<+]- < +[-<+]- ]lift 
                                                  +[->+]- > +[->+]- > +[->+]- (dp) > +[->+]- <<<< <<<<  -  ]rep  
    go into function while leaving open the progress 0s +[->+]- >[-]>[-]>
    write user ideas minus 10 until they press enter: its ascii is 10 ,----- -----[>,----- -----] 
    add the containment instructions 2 and 3 and 4 and the sector marker neg1 ++>+++>++++>-   go to the first of the fn's 0s   [<]<   ]! [+]-> #
]22: <+ [- noop SPACE [>] ] [+]-> 
]4: <+ [- The Containment Instructions (Loop)
    reset 4 0 0 to 2 3 4 and go back to the beginning; deleting the 2s that Lift have placed there >++++<+++<-- +[-<+]- > [-] > [-] ]4 [+]->
]3: <+ [- The Containment Instructions (Lift 2)
    delete the 3 < --- 
    lift over the second instruction into the slot just freed and run it and continue
    (placing a dummy 2 back there to not break the pointer system) +[-<+]- >> [  -  +[->+]- <<  +  +[-<+]- >> ] ++ [>] # ]3 [+]->
]2: <+ [- The Containment Instructions (Lift)
    delete the 2 < -- 
    lift over the first instruction into the slot just freed and run it and continue
    (placing a dummy 2 back there to not break the pointer system) +[-<+]- > [  -  +[->+]- <<<  +  +[-<+]- > ] ++ [>] # ]2 [+]-> 
]1: <+ [- noop 1 [>] ]1 [+]->
reset run flag and advance <+> > ]main
#
