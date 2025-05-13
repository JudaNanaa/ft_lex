%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

DIGIT [0-9]

%%
[A-Za-z][a-z-]+ printf("le nom est %s\n", yytext);
[0-9]+ printf("le nombre est %s\n", yytext);
"killer010" printf("ok"); 
%%