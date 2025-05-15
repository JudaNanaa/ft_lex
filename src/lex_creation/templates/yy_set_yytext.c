void yy_set_yytext(a_elem matching_state) {
	yyleng = matching_state.len_match;
	free(yytext);		
	yytext = malloc(sizeof(char) * (yyleng + 1));
	if (yytext == NULL)
		yy_fatal_error( "out of dynamic memory in set_yytext()" );
	memcpy(yytext, buffer.str, yyleng);
	yytext[yyleng] = '\0';
}
