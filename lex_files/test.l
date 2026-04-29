%{
#include <stdio.h>
%}

%%

^hello      { printf("BOL: ligne commencant par 'hello'\n"); }
world$      { printf("EOL: ligne finissant par 'world'\n"); }
^foo$       { printf("BOL+EOL: ligne exactement 'foo'\n"); }
[a-z]+      { printf("mot: %s\n", yytext); }
\n          ;
.           ;

%%

int main(void) {
    yylex();
    return 0;
}
