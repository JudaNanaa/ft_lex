%{
#include <stdio.h>
int line_num = 1;
%}

	test
%%
{test}  printf("trouve a\n");
