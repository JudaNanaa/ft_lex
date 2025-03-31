%x COMMENT TEST

%%

%{
	int a = 0;
%}

<COMMENT,TEST>{
	.* printf("ca a marche\n");
	test printf("je test\n"); BEGIN(INITIAL);
	ok printf("ca a marche\n"); BEGIN(TEST);
	ouijela printf("ca a marche\n");
}

[a-zA-Z]*@[a-zA-Z]*\.[a-z]{2,}    printf("EMAIL\n");
[0-9]{10}                         { printf("PHONE NUMBER\n"); }
[A-Za-z]*ing                      { printf("fini en ing0\n"); }
([A-Z])/e  ECHO; printf("chef %d\n", a++); BEGIN(COMMENT);
%%
