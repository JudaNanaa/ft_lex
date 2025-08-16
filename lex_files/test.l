%{
#include <stdio.h>
%}

%s COMMENT
%x STRING

%%

"/*"            BEGIN(COMMENT);
s< printf("yesbb\n");



<COMMENT>{
    "*/"        printf("end of comment\n"); BEGIN(INITIAL);
    .|\n        ;  // ignore le contenu du commentaire
}


<COMMENT,INITIAL,INITIAL>ok printf("test\n");

\"              BEGIN(STRING); // TODO: un probleme ici
<STRING>{
    \"          { BEGIN(INITIAL); printf("End of string\n"); }
    [^\"\n]+    { printf("String content: %s\n", yytext); }
    \n          { printf("Unterminated string\n"); BEGIN(INITIAL); }
}

"hello"    { printf("Hello detected\n"); }
<INITIAL>"bye"      { printf("Bye detected\n"); }

<*>[ \t\n]+     ; // ignore whitespace in all states

.               { printf("Other: %s\n", yytext); }

%%

int main(void) {
    yylex();
    return 0;
}
