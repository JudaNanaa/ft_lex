%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

test [0-9]

%%
ab*c printf("chef");
%%
