%{
#include <stdio.h>
%}

%array

%%

^hello      { printf("BOL: ligne commencant par 'hello'\n"); }
world$      { printf("EOL: ligne finissant par 'world'\n"); }
^foo$       { printf("BOL+EOL: ligne exactement 'foo'\n"); }

ab/cd       { printf("TRAILING: 'ab' suivi de 'cd' -> yytext=[%s]\n", yytext); }
[a-z]+      { printf("mot: %s\n", yytext); }
[0-9]+/\n   { printf("TRAILING: nombre en fin de ligne -> yytext=[%s]\n", yytext); }

\n          ;
.           ;

%%

int main(void) {
    yylex();
    return 0;
}
