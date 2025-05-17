int unput(int c) {
	if (buffer.len + 1 >= buffer.capacity) {
		buffer.str = realloc(buffer.str, (buffer.capacity * 2) + 1);
		if (buffer.str == NULL)
			yy_fatal_error( "out of dynamic memory in unput()" );
		buffer.capacity *= 2;
		memset(&buffer.str[buffer.len], 0, (buffer.capacity + 1) - buffer.len);
	}
	memmove(&buffer.str[yyleng + 1], &buffer.str[yyleng], buffer.len - yyleng);
	buffer.str[yyleng] = c;
	buffer.len += 1;
	buffer.str[buffer.len] = '\0';
	return 1;
}
