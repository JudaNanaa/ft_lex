void yy_set_yytext(a_elem matching_state) {
	if (yymore_flag == 0) {
		yyleng = matching_state.len_match;
		free(yytext);		
		yytext = malloc(sizeof(char) * (yyleng + 1));
		if (yytext == NULL)
			yy_fatal_error( "out of dynamic memory in set_yytext()" );
		memcpy(yytext, buffer.str, yyleng);
		yytext[yyleng] = '\0';
	}
	else {
		yytext = realloc(yytext, yyleng + matching_state.len_match + 1);
		if (yytext == NULL)
			yy_fatal_error( "out of dynamic memory in set_yytext()" );
		memcpy(&yytext[yyleng], buffer.str, matching_state.len_match);
		yyleng += matching_state.len_match;
		yytext[yyleng] = '\0';
		yymore_flag = 0;
	}
}
