%{
#include <stdio.h>
%}

%s COMMENT
%x STRING

test [0-9]

%%

ab/cd {printf("je suis en train de test\n");}

%%

int main(void) {
    yylex();
    return 0;
}