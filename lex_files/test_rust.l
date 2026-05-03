%%

^hello      { println!("BOL: ligne commencant par 'hello'"); }
world$      { println!("EOL: ligne finissant par 'world'"); }
^foo$       { println!("BOL+EOL: ligne exactement 'foo'"); }

ab/cd       { println!("TRAILING: 'ab' suivi de 'cd' -> yytext=[{}]", self.yytext); }
ok       	{ println!("ok pas mal"); }
[a-z]+      { println!("mot: {}", self.yytext); }
[0-9]+/\n   { println!("TRAILING: nombre en fin de ligne -> yytext=[{}]", self.yytext); }

\n          {}
.           {}

%%
