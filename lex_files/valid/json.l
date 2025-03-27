%{
#include <stdio.h>
%}

%%
"true"|"false"  printf("BOOLEAN: %s\n", yytext);
\"[^\"]*\"      printf("STRING: %s\n", yytext);
[0-9]+         printf("NUMBER: %s\n", yytext);
"{"            printf("OPEN_BRACE\n");
"}"            printf("CLOSE_BRACE\n");
":"            printf("COLON\n");
","            printf("COMMA\n");
[ \t\n]+       ; // Ignore espaces et retours à la ligne
.              printf("Invalid character: %c\n", *yytext);
%%

// int main() {
//     yylex();
//     return 0;
// }
