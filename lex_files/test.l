%{
#include <stdio.h>
int line_num = 1;
%}

%s test test
%x test t
%%
<test>a*{10}  printf("trouve a\n"); BEGIN(yes);
. BEGIN(test);
