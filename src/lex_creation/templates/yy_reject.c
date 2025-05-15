void yy_reject(void)
{
	if (stack.len == 0)
		return;

	yy_if_match();
}
