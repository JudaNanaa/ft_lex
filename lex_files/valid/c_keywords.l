%{
#include <stdio.h>
%}

%%

"whileif"      printf("KEYWORD: WHILEIF\n");
"while"      printf("KEYWORD: WHILE\n");
"if"      printf("KEYWORD: IF\n");
"else"    printf("KEYWORD: ELSE\n");
"return"  printf("KEYWORD: RETURN\n");
"("         printf("OPEN_PAREN\n");
")"         printf("CLOSE_PAREN\n");
"=="		printf("EQUAL SIGN\n");
";"		printf("SEMICOLON\n");
[a-zA-Z_][a-zA-Z0-9_]*  printf("IDENTIFIER: %s\n", yytext);
[0-9]+    printf("NUMBER: %s\n", yytext);
[ \t]+    ; // Ignore espaces
.         printf("Invalid character: %c\n", *yytext);
%%

int main() {
    yylex();
    return 0;
}
