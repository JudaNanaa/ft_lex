%{
#include <stdio.h>
%}

%%
[ \t\n]+            ; // Ignore espaces et retours à la ligne
\<[a-zA-Z0-9]+\>     printf("HTML_TAG_OPEN: %s\n", yytext);
\<\/[a-zA-Z0-9]+\>    printf("HTML_TAG_CLOSE: %s\n", yytext);
[^<>]+              printf("TEXT: %s\n", yytext);
.                   printf("Invalid character: %c\n", *yytext);
%%

int main() {
    yylex();
    return 0;
}
