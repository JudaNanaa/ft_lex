int yylex(void) {
	int current_state;
	int last_accepting_state;
	char *last_accepting_cpos = NULL;
	int c;
	int len_match;

	#write_in_yylex

	if (!yy_init)
	{
		yy_init = 1;

		if (yy_start == -1)
			yy_start = 0;

		if (!yyin)
			yyin = stdin;

		if (!yyout)
			yyout = stdout;

		if (!buffer.is_init)
			yy_init_buffer();
		yy_init_accepting_stack();
	}

	len_match = 0;
	current_state = yy_start;
	last_accepting_state = yy_start;

	

	while (1) {
		char *pos = yy_next_char();
		if (pos == NULL)
		{
			if (last_accepting_state == 0) {
				yy_if_no_match(last_accepting_cpos);
			}
			else
				yy_if_match();
			if (yywrap() == 0)
				yylex();
			break;
		}
		c = *pos;
		len_match++;
		unsigned char yy_c = yy_ec[YY_CHAR_TO_INT(c)];
		
		int next_state = yy_nxt[current_state][yy_c];
		if ( yy_accept[next_state] )
		{
			yy_search_final(next_state, len_match);
			last_accepting_state = next_state;
			last_accepting_cpos = pos;
		}
		if (next_state == 0)
		{
			if (last_accepting_state == 0) {
				yy_if_no_match(last_accepting_cpos);
			}
			else
				yy_if_match();
			last_accepting_cpos = 0;
			last_accepting_state = 0;
			current_state = 0;
			len_match = 0;
			clean_flag = 0;
		}
		current_state = next_state;
	}

	free(stack.t);
	stack.t = NULL;
	free(yytext);
	yytext = NULL;
	free(buffer.str);
	buffer.str = NULL;
	return 0;
}
