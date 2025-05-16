void yyless(int n) {
	char *dest;

	if (n > yyleng) {
		yy_fatal_error("n is bigger than length of yytext!");
	}

	dest = malloc(sizeof(char) * (n + 1));
	if (!dest)
		yy_fatal_error( "out of dynamic memory in set_yytext()" );
	memcpy(dest, yytext, n);
	dest[n] = '\0';
	free(yytext);
	yytext = dest;
	yyleng = n;
}
