%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

DIGIT [0-9]

%%
a printf("WORD: [%s]\n", yytext); ECHO; REJECT;
a* printf("test: [%s]\n", yytext);
a+ printf("ok: [%s]\n", yytext);
ab printf("test: [%s]\n", yytext);
ac printf("test: [%s]\n", yytext);
ad printf("test: [%s]\n", yytext);


%%