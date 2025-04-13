%{
#include <stdio.h>
int line_num = 1;
%}

%s test
%s comment

test [0-9]

%%
<test> {
	{
		{
			<comment>a printf("test\n");
			b printf("test\n");BEGIN(test);
		}
	}
}

. BEGIN(comment);
