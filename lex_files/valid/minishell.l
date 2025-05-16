%{
#include <stdio.h>
int line_num = 1;
%}

/* Modes optionnels, tu peux les utiliser si besoin */
%s test
%s comment

/* Définitions */
DIGIT      [0-9]
ID         [A-Za-z_][A-Za-z_0-9]*
WHITESPACE [ \t]+
NEWLINE    (\r\n|\r|\n)
STRING_SQ  \'([^']|\\')*\'
STRING_DQ  \"([^"]|\\\")*\"

%%
{NEWLINE}         { line_num++; }
{WHITESPACE}      ;  /* ignorer les espaces */
"#".*             ;  /* ignorer les commentaires shell-style */

{ID}              { printf("word: %s\n", yytext); }
{DIGIT}+          { printf("number: %s\n", yytext); }

"|"               { printf("pipe\n"); }
"||"              { printf("or\n"); }
"&&"              { printf("and\n"); }

"<"               { printf("redir_in\n"); }
"<<"              { printf("heredoc\n"); }
">"               { printf("redir_out\n"); }
">>"              { printf("append_out\n"); }

{STRING_SQ}       { printf("string (single quote): %s\n", yytext); }
{STRING_DQ}       { printf("string (double quote): %s\n", yytext); }

.                 { printf("unknown: %s\n", yytext); }

%%

int main(void) {
    printf("Enter a command:\n");
    yylex();
    return 0;
}
