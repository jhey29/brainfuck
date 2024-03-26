[ Memory Layout of a Scratch space 
1 0 iiiiI N 0 J ABCDEFGHiiiiiii etc 
An Item is 
0 data N scratch space data
where operations move around the scratch space relative to the data ]

read in the program 
+
read until double newline [
    make 01N >>+>-  
    go to first slot after the scratch space >>> >>>> >>>> +
        read until newline [ 
            read [-]>[-]>, duplicate [-<+<+>>] 
        < ----- ----- 
    ] < [-] delete the other newline copy
    < see if previous char was newline and is now also 0
]
go to second to last block (the last is empty) +[-<+]- < +[-<+]- 
go to I < 
[
copy I to J 
     [->>+>+<<<]>>> move I to 0 and J
    <[-<<+>>]> move back 0 to I; go to J
zero the scratch space from A to H: 
    >[-]- >>> >>>> +[[-]<+] < 
split instruction J into bits:
    do binary counting on reverse binary number: 01234567
    [-> [->] + +[-<+]- >>] #hello
    now on J = 0
do instruction
    >
    [bit 0    
        >
        [bit 1
            >
            [bit 2
                #7
            +[-<+]- >> ] < [ not bit 2 (bit 1 = 1) 
                #3 [-]
            ]
        +[-<+]- >> ] < [ not bit 1 (bit 0 = 1)
            set bit 1 >+
            > 
            [bit 2
                #5
            +[-<+]- >> ] < [ not bit 2 (bit 1 = 1) 
                #1 [-] noop: required by system
            ]
        ]
    odd: all these instructions move in the next byte; indirectly moving the scratch space
    +[-<+]  >[-]- >[-] >[-] >>>> >>>> [<<<< <<<< << <+> >> >>>> >>>>-]    +[-<+]- >> ] < [not bit 0
        set bit 0 >+
        >
        [bit 1
            >
            [bit 2 
                #6
            +[-<+]- >> ] < [ not bit 2 (bit 1 = 1) 
                #2 [-]
            ]
        +[-<+]- >> ] < [ not bit 1 (bit 0 = 1)
            set bit 1 >+ 
            >
            [bit 2
                #4
            +[-<+]- >> ] < [ not bit 2 (bit 1 = 1) 
                0 halt: handled by system [#0unreachable ]
            ]
        ]
    ]<< go to I
if I = 0 : end
]
#end