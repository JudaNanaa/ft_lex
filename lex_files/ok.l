%%
a     |
ab    |
abc   |
abcd  ECHO; REJECT;
.|\n  printf("xx%c", *yytext);
%%