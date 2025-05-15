void yy_increase_accepting_stack_len(void) {
	stack.t = realloc(stack.t, stack.capacity * 2);
	if (stack.t == NULL)
		yy_fatal_error( "out of dynamic memory in increase_accepting_stack_len()" );
	stack.capacity *= 2;
}
