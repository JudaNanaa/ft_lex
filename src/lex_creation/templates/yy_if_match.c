void yy_if_match() {
	a_elem matching_state = yy_pop_accepting_state();

	
	yy_set_yytext(matching_state);
	yy_action(matching_state.state);
	char *after_match = buffer.str + yyleng;
	if (clean_flag == 1)
		return;
	memmove(buffer.str, after_match, (&buffer.str[buffer.len]) - (after_match));
	bzero(&buffer.str[(&buffer.str[buffer.len]) - (after_match)], buffer.len - ((&buffer.str[buffer.len]) - (after_match)));
	buffer.len = &buffer.str[buffer.len] - (after_match);
	buffer.str[buffer.len] = '\0';
	buffer.index = 0;
	stack.len = 0;
	clean_flag = 1;
	// printf("remaining if_match = [%s]\n", buffer.str);
}
