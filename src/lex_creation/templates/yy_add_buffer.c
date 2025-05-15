char *yy_add_buffer(char c) {
	if (buffer.len == buffer.capacity) {
		buffer.str = realloc(buffer.str, buffer.capacity * 2);
		if (buffer.str == NULL)
			yy_fatal_error( "out of dynamic memory in add_buffer()" );
		buffer.capacity *= 2;
	}
	buffer.str[buffer.len] = c;
	buffer.len++;
	buffer.index++;
	return &buffer.str[buffer.len - 1];
}
