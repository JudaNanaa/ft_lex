%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

test [0-9]

%%
(0|1)* printf("chef");
1* printf("test");
%%

