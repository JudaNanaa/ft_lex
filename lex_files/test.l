%{
#include <stdio.h>
int line_num = 1;
%}
test yes

%a 1000000
%n 1000000
%%
.*{10000}  printf("trouve a\n");
