extern FILE *yyin, *yyout;
extern char *yytext;
extern int yyleng;

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

extern t_buffer buffer;

extern int yy_init;		/* whether we need to initialize */
extern int yy_start;	/* start state number */
extern int clean_flag;


change_me_in_variables!