/* Returned upon end-of-file. */
#define YY_NULL 0

/* Promotes a possibly negative, possibly signed char to an
 *   integer in range [0..255] for use as an array index.
 */
#define YY_CHAR_TO_INT(c) ((uint8_t) (c))

#define ECHO fwrite( yytext, (size_t) yyleng, 1, yyout );

