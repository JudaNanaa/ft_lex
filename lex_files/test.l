%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

DIGIT [0-9]

%%
	printf("chef\n");
"quote\\\"test" printf("ok\n");
%%

int main(void) {
	yylex();
}