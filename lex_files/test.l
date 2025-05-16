%{
#include <stdio.h>
%}

%%
test        { yyless(1); int c = input(); printf("ok [%c]\n", c); }               // préparer à ajouter le suivant
ok          { printf("ok [%s]\n", yytext); } // imprimera "testok"
%%

int main(void) {
    yylex();
    return 0;
}
