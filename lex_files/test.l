%{
#include <stdio.h>
%}

%%
test        { yymore(); }               // préparer à ajouter le suivant
ok          { printf("ok [%s]\n", yytext); } // imprimera "testok"
.|\n        ;   // ignorer le reste
%%

int main(void) {
    yylex();
    return 0;
}
