void yy_init_accepting_stack(void) {
	stack.t = calloc(sizeof(a_elem), MIN_CAPACITY);
	if (buffer.str == NULL)
		yy_fatal_error( "out of dynamic memory in init_accepting_stack()" );
	buffer.capacity = MIN_CAPACITY;
	buffer.len = 0;
}
