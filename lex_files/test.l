%{
#include <stdio.h>
%}

%%
test        { yyless(0); printf("ok [%s]\n", yytext); }               // préparer à ajouter le suivant
ok          { printf("ok [%s]\n", yytext); } // imprimera "testok"
%%

int main(void) {
    yylex();
    return 0;
}
