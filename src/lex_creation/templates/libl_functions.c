#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#ifndef YY_EXIT_FAILURE
#define YY_EXIT_FAILURE 2
#endif
#define YY_CHAR_TO_INT(c) ((uint8_t) (c))


extern const unsigned char yy_ec[256];           // tableau externe
extern const unsigned int yy_nxt[][256]; // ici, 256 est connu               // transition de l'automate
extern const int yy_accept[];               // états finaux

void yy_search_final(int state, int len);
void yy_if_match(void);
void yy_if_no_match(char *cpos);
void yy_action(int state);
int yylex(void);

#define MIN_CAPACITY 1024

FILE *yyin = NULL, *yyout = NULL;

typedef struct accept_elem {
	int state;
	size_t len_match;
} a_elem;

typedef struct accept_stack {
	a_elem *t;
	size_t len;
	size_t capacity;
} a_stack;

static a_stack stack = {0};

typedef struct s_buffer {
	char *str;
	size_t len;
	size_t capacity;
	size_t index;
	uint8_t is_init;
} t_buffer;

char *yytext;
int yyleng;
static int clean_flag = 0;
static int yymore_flag = 0;

static t_buffer buffer;


static int yy_init = 0;		/* whether we need to initialize */
static int yy_start = -1;	/* start state number */

void yy_fatal_error (const char* msg )
{
	fprintf( stderr, "%s\n", msg );
	exit( YY_EXIT_FAILURE );
}
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
void yy_init_accepting_stack(void) {
	stack.t = calloc(sizeof(a_elem), MIN_CAPACITY);
	if (stack.t == NULL)
		yy_fatal_error( "out of dynamic memory in init_accepting_stack()" );
	stack.capacity = MIN_CAPACITY;
	stack.len = 0;
}
void yy_increase_accepting_stack_len(void) {
	stack.t = realloc(stack.t, stack.capacity * 2);
	if (stack.t == NULL)
		yy_fatal_error( "out of dynamic memory in increase_accepting_stack_len()" );
	stack.capacity *= 2;
}
void yy_push_accepting_state(int state, int len_match) {
	int index = stack.len;

	if (stack.len == stack.capacity)
		yy_increase_accepting_stack_len();
	stack.t[index].state = state;
	stack.t[index].len_match = len_match;
	stack.len++;
}
a_elem yy_pop_accepting_state(void) {
	a_elem pop;

	int index = stack.len - 1;

	pop.state = stack.t[index].state;
	pop.len_match = stack.t[index].len_match;

	stack.len--;
	return pop;
}
char *yy_add_buffer(char c) {
	if (buffer.len == buffer.capacity) {
		buffer.str = realloc(buffer.str, buffer.capacity * 2 + 1);
		if (buffer.str == NULL)
			yy_fatal_error( "out of dynamic memory in add_buffer()" );
		memset(&buffer.str[buffer.len], 0, buffer.capacity - buffer.len);
		buffer.capacity *= 2;
	}
	buffer.str[buffer.len] = c;
	buffer.len++;
	buffer.str[buffer.len] = '\0';
	buffer.index++;
	return &buffer.str[buffer.len - 1];
}
void yy_if_no_match(char *last_pos) {

	// printf("not match [%s]\n", buffer.str);
	
	if (last_pos == NULL)
		last_pos = buffer.str;
	fwrite(buffer.str, sizeof(char), last_pos + 1 - buffer.str, yyout);
	int n = (&buffer.str[buffer.len]) - (last_pos + 1);
	if (n < 0)
		n = 0;
	memmove(buffer.str, last_pos + 1, n);
	bzero(&buffer.str[n], buffer.len - (n));
	buffer.len = n;
	buffer.str[buffer.len] = '\0';
	buffer.index = 0;
	// printf("remaining if_no_match = [%s]\n", buffer.str);
}
void yy_set_yytext(a_elem matching_state) {
	if (yymore_flag == 0) {
		yyleng = matching_state.len_match;
		free(yytext);		
		yytext = malloc(sizeof(char) * (yyleng + 1));
		if (yytext == NULL)
			yy_fatal_error( "out of dynamic memory in set_yytext()" );
		memcpy(yytext, buffer.str, yyleng);
		yytext[yyleng] = '\0';
	}
	else {
		yytext = realloc(yytext, yyleng + matching_state.len_match + 1);
		if (yytext == NULL)
			yy_fatal_error( "out of dynamic memory in set_yytext()" );
		memcpy(&yytext[yyleng], buffer.str, matching_state.len_match);
		yyleng += matching_state.len_match;
		yytext[yyleng] = '\0';
		yymore_flag = 0;
	}
}
char *yy_next_char(void) {
	int c;
	
	if (buffer.str[buffer.index] == '\0')
	{
		c = getc(yyin);
		if (c == EOF)
			return NULL;
		return yy_add_buffer(c);
	}
	char *pos = &buffer.str[buffer.index];
	buffer.index++;
	return pos;
}
int yymore(void) {
	yymore_flag = 1;
	return 1;
}
int yyless(int n) {
	char *dest;

	if (n > yyleng) {
		yy_fatal_error("n is bigger than length of yytext!");
	}

	dest = malloc(sizeof(char) * (n + 1));
	if (!dest)
		yy_fatal_error( "out of dynamic memory in set_yytext()" );
	memcpy(dest, yytext, n);
	dest[n] = '\0';
	free(yytext);
	yytext = dest;
	yyleng = n;
	return 1;
}
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
int unput(int c) {
	if (buffer.len + 1 >= buffer.capacity) {
		buffer.str = realloc(buffer.str, (buffer.capacity * 2) + 1);
		if (buffer.str == NULL)
			yy_fatal_error( "out of dynamic memory in unput()" );
		buffer.capacity *= 2;
		memset(&buffer.str[buffer.len], 0, (buffer.capacity + 1) - buffer.len);
	}
	memmove(&buffer.str[yyleng + 1], &buffer.str[yyleng], buffer.len - yyleng);
	buffer.str[yyleng] = c;
	buffer.len += 1;
	buffer.str[buffer.len] = '\0';
	return 1;
}

// default_yywrap
__attribute__((weak))
int yywrap(void) {
	return 1;
}

void yy_if_match() {
	a_elem matching_state = yy_pop_accepting_state();

	
	yy_set_yytext(matching_state);
	yy_action(matching_state.state);
	char *after_match = buffer.str + yyleng;
	if (clean_flag == 1)
		return;
	memmove(buffer.str, after_match, (&buffer.str[buffer.len]) - (after_match));
	bzero(&buffer.str[(&buffer.str[buffer.len]) - (after_match)], buffer.len - ((&buffer.str[buffer.len]) - (after_match)));
	buffer.len = &buffer.str[buffer.len] - (after_match);
	buffer.str[buffer.len] = '\0';
	buffer.index = 0;
	stack.len = 0;
	clean_flag = 1;
	// printf("remaining if_match = [%s]\n", buffer.str);
}
void yy_reject(void)
{
	if (stack.len == 0)
		return;

	yy_if_match();
}

// default_main.c
__attribute__((weak))
int main(void) {
    yylex();
    return 0;
}