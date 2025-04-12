%{
#include <stdio.h>
int line_num = 1;
%}

%s test 

%%
%%sss printf("yes\n");
.*{1000}  {while(1) printf("a\n");};
