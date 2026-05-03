pub struct Lexer<R: std::io::Read> {
    core: ft_lex_runtime::LexerCore<R>,
}

impl<R: std::io::Read> std::ops::Deref for Lexer<R> {
    type Target = ft_lex_runtime::LexerCore<R>;
    fn deref(&self) -> &Self::Target { &self.core }
}

impl<R: std::io::Read> std::ops::DerefMut for Lexer<R> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.core }
}

