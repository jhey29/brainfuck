0 is charcode 48
>- MARK Beginning 
>>+++++ +[-<+++++ +>]<++ generate 38
[>,----- -----] read input minus 10 until enter
-< MARK Number End
+[-<+]- go to Beginning 
>[->+[-->+]- < +[-<+]->] subtract 38 from all cells to the right until Number End
>> go to second cell of number
until Number End 
+[-<[->+++++ +++++<]>>+]- multiply number to the right by 10 and add on top 
<. #err0
|. op 0 
- set Marker
>, input 
+[-
[ not on 0
-[->+>+<<]>>[-<<+>>]<
 > . <  op 0
] on 0  
+ . - <  1 and go left
+]- until Marker
|
>+++++[-<+++++ +++>] generate 40 
+[, [-<+>>+<] < . >>[-<<->>] <+] #err get bit output and reset ad inf

