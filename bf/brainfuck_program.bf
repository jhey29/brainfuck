setup
+++++ +++++ ++ (11)


    rRCO        ::
    0rRCO       :: etc etc
    01234567RCO :: 10 bytes required
dissect the instruction at the main ptr into the seven to eight zeroes to its right by 
copying it over to one cell R and increasing a second cell C kept mod 2  
and subtracting every time it is 0 (utilizing the O to its right); thereby dividing by 2; 
then moving that carry bit over to the spot left from copying into R; then repeating 
copy  read carry and remember    increase carry if every first time  //yay no conditional positioning 
                                 clear if every second time
[[->+>-  [+<->>-<]                    +>+ [<->-]   <<< ]
+>>[<<->>-]< # ] -






#
