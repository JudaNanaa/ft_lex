%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

DIGIT [0-9]

%%
a     |
ab    |
abc   |
abcd  ECHO; REJECT;
.|\n  printf("xx%c", *yytext);
%%