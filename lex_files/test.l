%{
#include <stdio.h>
%}

%s COMMENT
%x STRING

%%

ok printf("test de initial\n") ;BEGIN(COMMENT);
revient printf("revient a initial initial\n") ;BEGIN(0);
<COMMENT>.+ printf("test de comment\n");
