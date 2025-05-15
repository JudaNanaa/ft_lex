%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment

DIGIT [0-9]

%%
[0-9]+     printf("NUMBER: %s\n", yytext);
"+"|"-"|"*"|"/"  printf("OPERATOR: %s\n", yytext);
"("         printf("OPEN_PAREN\n");
")"         printf("CLOSE_PAREN\n");
[ \t]+     ;
.          printf("Invalid character: %c\n", *yytext);
%%

int main(void) {
	yylex();
}