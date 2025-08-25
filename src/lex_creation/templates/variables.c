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

static t_buffer buffer;

static int yy_init = 0;		/* whether we need to initialize */
static int yy_start = -1;	/* start state number */
static int clean_flag = 0;


change_me_in_variables!