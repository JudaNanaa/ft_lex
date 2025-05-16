%{
#include <stdio.h>
int line_num = 1;
%}
%s test
%s comment
	// okoo
DIGIT [0-9]

%%
[A-Za-z_][A-Za-z_0-9]* printf("word\n");
"|" printf("pipe\n");
"<"|"<<"|">>"|">" printf("redirection\n");
"&&" printf("and\n");
"||" printf("or\n");
[ \t] ;

%%

int main(int argc, char **argv) {
	if (argc == 2)
		yyin = fopen(argv[1], "r");
	yylex();
	fclose(yyin);
}