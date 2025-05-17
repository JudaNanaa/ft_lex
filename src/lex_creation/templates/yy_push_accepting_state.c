void yy_push_accepting_state(int state, int len_match) {
	int index = stack.len;

	if (stack.len == stack.capacity)
		yy_increase_accepting_stack_len();
	stack.t[index].state = state;
	stack.t[index].len_match = len_match;
	stack.len++;
}
