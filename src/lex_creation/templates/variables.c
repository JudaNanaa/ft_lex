
FILE *yyin = NULL, *yyout = NULL;

extern int yylineno;
int yylineno = 1;

static int yy_init = 0;		/* whether we need to initialize */
static int yy_start = 0;	/* start state number */
