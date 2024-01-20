 I N 0 5
  >-> >++++  +
[
split instruction into bits:
    do binary counting on reverse binary number: 01234567
    [-> [->] + +[-<+]- >>] #hello
    pointer always 2
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
                #1 [-]
            ]
        ]
    +[-<+]- >> ] < [not bit 0
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
                4
            +[-<+]- >> ] < [ not bit 2 (bit 1 = 1) 
                0: halt [#0unreachable ]
            ]
        ]
    ]<
move out next instruction from behind N 
<[->>>+<<<]>>>
if instruction = 0 : end
#end 
]
