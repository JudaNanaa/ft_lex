%{
#include <stdio.h>
%}

%s COMMENT
%x STRING

test [0-9]

%%

a+/b {printf("je suis en train de test\n");}

ab/c {printf("je suis en train de testl\n");}

%%

int main(void) {
    yylex();
    return 0;
}