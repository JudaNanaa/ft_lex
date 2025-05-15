a_elem yy_pop_accepting_state(void) {
	a_elem pop;

	int index = stack.len - 1;

	pop.state = stack.t[index].state;
	pop.len_match = stack.t[index].len_match;

	stack.len--;
	return pop;
}