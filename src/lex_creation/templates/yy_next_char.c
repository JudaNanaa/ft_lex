char *yy_next_char(void) {
	int c;
	
	if (buffer.str[buffer.index] == '\0')
	{
		c = getc(yyin);
		if (c == EOF)
			return NULL;
		return yy_add_buffer(c);
	}
	char *pos = &buffer.str[buffer.index];
	buffer.index++;
	return pos;
}
