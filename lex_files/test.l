%{
#include <stdio.h>
int line_num = 1;
%}

%s test
%s comment

test [0-9]

%%
	printf("test\n");
dc |
a|b printf("yes");
