[ build the first data section (-1 at  address 0, 36, 45)  ] 
->+++++ +++++ ++[[>>>+<<<-]>>>-]<->>>>> >>>>- the data pointer neg1 at address 33 complicates this
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
38-<<+>>[ 39-<<+>>[ 40-<<+>>[ 41-<<+>>[ 42-<<+>>[ 43-<<+>>[ 44-<<+>>[ 45-<<+>>[ 46-<<+>>[ 47-<<+>>[ 48-<<+>>[ 49-<<+>>[ 50-<<+>>[ 51-<<+>>[ 52-<<+>>[ 53-<<+>>[ 54-<<+>>[ 
55-<<+>>[ 56-<<+>>[ 57-<<+>>[ 58-<<+>>[ 59-<<+>>[ 60-<<+>>[ 61-<<+>>[ 62-<<+>>[ 63-<<+>>[ 64-<<+>>[ 65-<<+>>[ 66-<<+>>[ 67-<<+>>[ 68-<<+>>[
69-<<+>>[ 70-<<+>>[ 71-<<+>>[ 72-<<+>>[ 73-<<+>>[ 74-<<+>>[ 75-<<+>>[ 76-<<+>>[ 77-<<+>>[ 78-<<+>>[ 79-<<+>>[ 80-<<+>>[ 81-<<+>>[

[commands implemented: :;76543210<>+-# todo: @? ]
tbd!
for each instruction: we are sitting on the second 0 we just made anew  go to the run flag  add one to the run flag to make self run   
    if it runs it must: {reduce the one again  do stuff   
    return to the run flag : the first 0  to break the 'loop'}    
set the run flag to neg1 so the other instrs don't run  go back to the ppp zero to make the switch loop fall through
]81:open <+ [- noop [>] ]open [+]->
]80:Z <+ [- noop [>] ]Z [+]->
]79:Y <+ [- noop [>] ]Y [+]-> 
]78:X <+ [- noop [>] ]X [+]-> XOR ?
]77:W <+ [- noop [>] ]W [+]->
]76:V <+ [- noop [>] ]V [+]->
]75:U <+ [- noop [>] ]U [+]->
]74:T <+ [- noop [>] ]T [+]-> sum mask-selected counters into Sc
]73:S <+ [- noop [>] ]S [+]-> 
]72:R <+ [- noop [>] ]R [+]-> SHR
]71:Q <+ [- noop [>] ]Q [+]->
]70:P <+ [- noop [>] ]P [+]->
]69:O <+ [- noop [>] ]O [+]-> OR
]68:N <+ [- noop [>] ]N [+]->
]67:M <+ [- noop [>] ]M [+]-> masked add Sc into counters
]66:L <+ [- noop [>] ]L [+]-> ASL
]65:K <+ [- noop [>] ]K [+]->
]64:J <+ [- noop [>] ]J [+]-> DEC
]63:I <+ [- noop [>] ]I [+]-> INC
]62:H <+ [- noop [>] ]H [+]->
]61:G <+ [- noop [>] ]G [+]->
]60:F <+ [- noop [>] ]F [+]->
]59:E <+ [- noop [>] ]E [+]-> EOR ?
]58:D <+ [- noop [>] ]D [+]-> BCD ?
]57:C <+ [- noop [>] ]C [+]-> convert into Sc 
]56:B <+ [- noop [>] ]B [+]-> convert from Sc
]55:A <+ [- noop [>] ]A [+]-> AND
]54:@ <+ [- store *close dp R into *far dp R :  [>] ]@ [+]-> TODO
]53:? <+ [- load *far dp R into *close dp R : set close bit 7 (as destroying bit pointer) to m1 and zero other R bits +[-<+]- < +[-<+]- >>>> >>>> [-]- < +[[-]<+]- < 
    go set far carry (as shifting bit pointer) to m1 +[-<+]- >>>> >>>> > [-]-
    until far dp reached <+[- 
        if not 0  [ go to close bip >> +[->+]- > +[->+]- set 1 [-]+ advance <[-]-  < go back  +[-<+]- < +[-<+]- shift [+] <[->+<] ] there is now data to the right ; 0 ; and OptionData to the left 
        if next part would be dumb to do since already at far dp; set far sbip to 1; concretely: if OptionData not m1:  [>+]
        if not 1 -[+go to close m1p >> +[->+]- > +[->+]- set 0 [-]  advance <[-]-  < go back  +[-<+]- < +[-<+]- advance [+] <[->+<] -]+
    ]  [>] ]? [+]->TODO
]52:gt <+ [- move dp to the right 9 +[-<+]- < +[-<+]- + >>>> >>>> > [<<<< <<<< <] - > +[->+]- [>] ]gt [+]-> safety: does nothing on oob move
]51:= <+ [- noop [>] ]= [+]->
]50:lt <+ [- move dp to the left 9 +[-<+]- < +[-<+]- + <<<< <<<< < [>>>> >>>> >] - > +[->+]- [>] ]lt [+]->
]49:; <+ [- delete closest dp +[-<+]- < +[-<+]- + > +[->+]- [>] ]; [+]->
]48:: <+ [- spawn new close dp +[-<+]- <<<< <<<< < - > +[->+]- [>] ]: [+]-> 
]47:9 <+ [- noop [>] ]9 [+]->
]46:8 <+ [- noop [>] ]8 [+]->
]45:7 <+ [- set bit 7 in L +[-<+]- < +[-<+]-  <[-]+>   > +[->+]-  [>] ]7 [+]-> the integers are lsb left despite the dp direction being right because I thought this'd make carry easier
]44:6 <+ [- set bit 6 in L +[-<+]- < +[-<+]-  <<[-]+>>   > +[->+]-  [>] ]6 [+]->
]43:5 <+ [- set bit 5 in L +[-<+]- < +[-<+]-  <<<[-]+>>>   > +[->+]-  [>] ]5 [+]->
]42:4 <+ [- set bit 4 in L +[-<+]- < +[-<+]-  <<<<[-]+>>>>   > +[->+]-  [>] ]4 [+]->
]41:3 <+ [- set bit 3 in L +[-<+]- < +[-<+]- <<<< <[-]+> >>>>  > +[->+]-  [>] ]3 [+]->
]40:2 <+ [- set bit 2 in L +[-<+]- < +[-<+]- <<<< <<[-]+>> >>>>  > +[->+]-  [>] ]2 [+]->
]39:1 <+ [- set bit 1 in L +[-<+]- < +[-<+]- <<<< <<<[-]+>>> >>>>  > +[->+]-  [>] ]1 [+]->
]38:0 <+ [- set bit 0 in L +[-<+]- < +[-<+]- <<<< <<<<[-]+>>>> >>>>  > +[->+]-  [>] ]0 [+]->
]37:/ <+ [- noop [>] ]/ [+]->
]36:dot <+ [- noop [>] ]dot [+]->
]35:minus <+ [- twos complement subtraction L into R zeroing L: +[-<+]- < +[-<+]- >>>> >>>> > set the carry to minus 1 for later orientation [-]- < +[-<+]- <<<< <<<< (l0)       
    until dp reached: first dec each bit in L by one +[-->+]-   change the dp to 0 temporarily + #
    then add 'one' to L <<<< <<<< [+>]- #    change the dp to 1 by going to the 'carry' we don't need yet and changing it back to 0  >>>> >>>> +[->+] <<<< <<<< < [-]+ <<<< <<<< (l0)
    until dp reached: for each minus 1: go to corresponding bit in R and add with carry  -[+ [+>>>> >>>> > [->]+ <<<< <<<< < -[+>-]+ <<<< <<<<] > -]+ chg back dp -- >  +[->+]- [>] ]minus [+]->
]34:comma <+ [- noop [>] ]comma [+]->
]33:plus <+ [- add L into R (close dp) zeroing L: +[-<+]- < +[-<+]- <<<< <<<< (l0)
    until dp reached: for each 1: go to corresponding bit in R and add with carry  +[- [->>>> >>>> > [->]+ +[-<+]- <<<< <<<<] > +]- > +[->+]-  [>] ]plus [+]->
]32:* <+ [- noop [>] ]* [+]->
]31:) <+ [- noop [>] ]) [+]->
]30:( <+ [- noop [>] ]( [+]->
]29:' <+ [- freeze closest live dp (make it minus 2) +[-<+]- < +[-<+]- - +[->+]- [>] ]' [+]->
]28:& <+ [- noop [>] ]& [+]-> 
]27:% <+ [- noop [>] ]% [+]->
]26:$ <+ [- halt >>[-]<< [>] ]$ [+]->
]25:hash <+ [- debug # [>] ]hash [+]->
]24:" <+ [- unfreeze furthest frozen dp before closest live dp +[-<+]- < +[-<+]- ++[-->++]-- + > +[->+]- [>] ]" [+]->
]23:! <+ [- Write user function offset Sc no malloc:  go to Sc  +[-<+]- <<<< <<<<
    Sc transport:repeat for offset [
    Sc transport:lift Sc one data pool over [  -  +[->+]- > +[->+]- > +[->+]- (dp) > +[->+]- <<<< <<<<  +  +[-<+]- < +[-<+]- < +[-<+]- ]lift 
                                                  +[->+]- > +[->+]- > +[->+]- (dp) > +[->+]- <<<< <<<<  -  ]rep  
    go into function while leaving open the progress 0s +[->+]- >[-]>[-]>
    write user ideas minus 10 until they press enter: its ascii is 10 ,----- -----[>,----- -----] 
    add the containment instructions 2 and 3 and 4 and the sector marker neg1 ++>+++>++++>-   go to the first of the fn's 0s   [<]<   ]! [+]-> #
]22:space <+ [- noop [>] ] [+]-> 
]4: <+ [- The Containment Instructions (Loop)
    reset 4 0 0 to 2 3 4 and go back to the beginning; deleting the 2s that Lift have placed there >++++<+++<-- +[-<+]- > [-] > [-] < ]4 [+]->
]3: <+ [- The Containment Instructions (Lift 2)
    delete the 3 < --- 
    lift over the second instruction into the slot just freed and run it and continue
    (placing a dummy 2 back there to not break the pointer system) +[-<+]- >> [  -  +[->+]- <<  +  +[-<+]- >> ] ++ [>] ]3 [+]->
]2: <+ [- The Containment Instructions (Lift)
    delete the 2 < -- 
    lift over the first instruction into the slot just freed and run it and continue
    (placing a dummy 2 back there to not break the pointer system) +[-<+]- > [  -  +[->+]- <<<  +  +[-<+]- > ] ++ [>] ]2 [+]-> 
]1: <+ [- noop 1 [>] ]1 [+]->
reset run flag and advance <+> > ]main

