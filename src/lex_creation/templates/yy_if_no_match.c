void yy_if_no_match(char *last_pos) {

	// printf("not match [%s]\n", buffer.str);
	
	if (last_pos == NULL)
		last_pos = buffer.str;
	fwrite(buffer.str, sizeof(char), last_pos + 1 - buffer.str, yyout);
	memmove(buffer.str, last_pos + 1, (&buffer.str[buffer.len]) - (last_pos + 1));
	bzero(&buffer.str[(&buffer.str[buffer.len]) - (last_pos + 1)], buffer.len - ((&buffer.str[buffer.len]) - (last_pos + 1)));
	buffer.len = (&buffer.str[buffer.len]) - (last_pos + 1);
	buffer.str[buffer.len] = '\0';
	buffer.index = 0;
	// printf("remaining if_no_match = [%s]\n", buffer.str);
}
