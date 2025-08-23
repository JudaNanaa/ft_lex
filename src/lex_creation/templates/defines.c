/* Returned upon end-of-file. */
#define YY_NULL 0

/* Promotes a possibly negative, possibly signed char to an
 *   integer in range [0..255] for use as an array index.
 */
#define YY_CHAR_TO_INT(c) ((uint8_t) (c))
#define MIN_CAPACITY 1024

#define ECHO fwrite( yytext, (size_t) yyleng, 1, yyout );

#define REJECT yy_reject();
void yy_reject(void);

#ifndef YY_EXIT_FAILURE
#define YY_EXIT_FAILURE 2
#endif

#define INITIAL 0
