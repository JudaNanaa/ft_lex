void yy_init_buffer(void)
{
	buffer.str = calloc(sizeof(char), MIN_CAPACITY + 1);
	if (buffer.str == NULL)
		yy_fatal_error( "out of dynamic memory in init_buffer()" );
	buffer.capacity = MIN_CAPACITY;
	buffer.len = 0;
	buffer.index = 0;
	buffer.is_init = 1;
}
