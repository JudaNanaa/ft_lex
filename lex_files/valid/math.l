%{
#include <stdio.h>
%}

%%
[0-9]+     printf("NUMBER: %s\n", yytext);
"+"|"-"|"*"|"/"  printf("OPERATOR: %s\n", yytext);
"("         printf("OPEN_PAREN\n");
")"         printf("CLOSE_PAREN\n");
[ \t]+     ;
.          printf("Invalid character: %c\n", *yytext);
%%

int main() {
    yylex();
    return 0;
}
