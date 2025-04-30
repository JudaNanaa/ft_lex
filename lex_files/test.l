%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

DIGIT [0-9]

%%
dadadadadadadad printf("NUMBER: %s\n", yytext);
%%
