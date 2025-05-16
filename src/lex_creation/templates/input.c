int input(void) {
	int c;

	if (buffer.str[yyleng]) {
		c = buffer.str[yyleng];
		memmove(&buffer.str[yyleng], &buffer.str[yyleng + 1], buffer.len - (yyleng + 1));
		return c;
	}
	c = getc(yyin);
	if (c == EOF)
		c = 0;
	return c;
}
