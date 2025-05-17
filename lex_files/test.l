%{
#include <stdio.h>
%}

%%
test        { printf("ok [%s]\n", yytext); unput('k'); unput('o'); unput('k'); unput('o'); unput('k'); unput('o'); printf("ok [%s]\n", yytext);}               // préparer à ajouter le suivant
ok          { printf("chef [%s]\n", yytext); } // imprimera "testok"
%%

int main(void) {
    yylex();
    return 0;
}
