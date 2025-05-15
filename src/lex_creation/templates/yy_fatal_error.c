static void yy_fatal_error (const char* msg )
{
	fprintf( stderr, "%s\n", msg );
	exit( YY_EXIT_FAILURE );
}
