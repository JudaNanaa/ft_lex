char *yy_add_buffer(char c) {
	printf("okoko\n");
	if (buffer.len == buffer.capacity) {
		buffer.str = realloc(buffer.str, buffer.capacity * 2 + 1);
		if (buffer.str == NULL)
			yy_fatal_error( "out of dynamic memory in add_buffer()" );
		memset(&buffer.str[buffer.len], 0, buffer.capacity - buffer.len);
		buffer.capacity *= 2;
	}
	buffer.str[buffer.len] = c;
	buffer.len++;
	buffer.index++;
	return &buffer.str[buffer.len - 1];
}
