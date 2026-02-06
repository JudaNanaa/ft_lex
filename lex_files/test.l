%{
#include <stdio.h>
%}

%s COMMENT
%x STRING

test [0-9]

%%

l|r/g {printf("je suis en train de test\n");}

ab/dc {printf("je suis en train de testl\n");}

%%

int main(void) {
    yylex();
    return 0;
}