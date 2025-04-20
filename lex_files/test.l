%{
#include <stdio.h>
int line_num = 1;
%}

%s test
%s comment

test [0-9]

%%
	printf("test\n");
a{1,3
a{13}* printf("test\n");
{test}{test}* printf("test\n");
a{1,}* printf("test\n");
