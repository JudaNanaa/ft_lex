void yy_init_accepting_stack(void) {
	stack.t = calloc(sizeof(a_elem), MIN_CAPACITY);
	if (stack.t == NULL)
		yy_fatal_error( "out of dynamic memory in init_accepting_stack()" );
	stack.capacity = MIN_CAPACITY;
	stack.len = 0;
}
