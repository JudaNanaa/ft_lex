
FILE *yyin = NULL, *yyout = NULL;

extern int yylineno;
int yylineno = 1;

static int yy_init = 0;		/* whether we need to initialize */
static int yy_start = -1;	/* start state number */

char *yytext;
int yyleng;
static int clean_flag = 0;
static int yymore_flag = 0;

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

change_me_in_variables!