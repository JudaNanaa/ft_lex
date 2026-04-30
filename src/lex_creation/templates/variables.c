extern FILE *yyin, *yyout;
extern char *yytext;
extern int yyleng;

typedef struct accept_elem {
	int state;
	int dfa_state;
	size_t len_match;
} a_elem;

typedef struct accept_stack {
	a_elem *t;
	size_t len;
	size_t capacity;
} a_stack;


typedef struct s_buffer {
	char *str;
	size_t len;
	size_t capacity;
	size_t index;
	uint8_t is_init;
} t_buffer;

extern a_stack stack;
extern t_buffer buffer;

extern int yy_init;		/* whether we need to initialize */
extern int yy_start;	/* start state number */
extern int clean_flag;
extern int yy_trailing_len;
extern const int yy_trailing[];
extern const int yy_trailing_accept[];


change_me_in_variables!