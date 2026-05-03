impl<R: std::io::Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Self {
            input,
            yytext: String::new(),
            yyleng: 0,
            yy_start: 0,
            yy_init: false,
            buffer: Vec::new(),
            buffer_index: 0,
            stack: Vec::new(),
            clean_flag: false,
            yymore_flag: false,
            yy_trailing_len: 0,
            yy_at_bol: true,
        }
    }

    pub fn begin(&mut self, state: usize) {
        self.yy_start = state;
    }

    fn yy_next_char(&mut self) -> Option<u8> {
        if self.buffer_index < self.buffer.len() {
            let c = self.buffer[self.buffer_index];
            self.buffer_index += 1;
            return Some(c);
        }
        let mut buf = [0u8; 1];
        match self.input.read(&mut buf) {
            Ok(1) => {
                self.buffer.push(buf[0]);
                self.buffer_index += 1;
                Some(buf[0])
            }
            _ => None,
        }
    }

    fn yy_search_final(&mut self, state: usize, len_match: usize) {
        let mut i = 0;
        while YY_ACCEPT_FLAT[state * YY_ACCEPT_COLS + i] != 0 {
            self.stack.push(AcceptElem {
                action: YY_ACCEPT_FLAT[state * YY_ACCEPT_COLS + i],
                dfa_state: state,
                len_match,
            });
            i += 1;
        }
    }

    fn yy_pop_accepting_state(&mut self) -> AcceptElem {
        self.stack.pop().expect("pop on empty accepting stack")
    }

    fn yy_finish_state(&self, state: usize) -> bool {
        (0..YY_NXT_COLS).any(|c| YY_NXT_FLAT[state * YY_NXT_COLS + c] != 0)
    }

    fn yy_set_yytext(&mut self, elem: &AcceptElem) {
        if !self.yymore_flag {
            self.yyleng = elem.len_match;
            self.yytext = String::from_utf8_lossy(&self.buffer[..self.yyleng]).into_owned();
        } else {
            let extra = &self.buffer[self.yyleng..self.yyleng + elem.len_match];
            self.yytext.push_str(&String::from_utf8_lossy(extra));
            self.yyleng += elem.len_match;
            self.yymore_flag = false;
        }
    }

    fn yy_if_match(&mut self) -> Option<i32> {
        let matching = self.yy_pop_accepting_state();
        if self.yy_trailing_len > 0 && YY_TRAILING_ACCEPT[matching.dfa_state] != 0 {
            let tc = AcceptElem {
                action: matching.action,
                dfa_state: matching.dfa_state,
                len_match: self.yy_trailing_len,
            };
            self.yy_set_yytext(&tc);
            self.yy_trailing_len = 0;
        } else {
            self.yy_set_yytext(&matching);
        }
        let action = matching.action;
        self.yy_at_bol = self.yytext.ends_with('\n');
        let ret = self.yy_action(action);
        if !self.clean_flag {
            let remaining = self.buffer[self.yyleng..].to_vec();
            self.buffer = remaining;
            self.buffer_index = 0;
            self.stack.clear();
            self.clean_flag = true;
        }
        ret
    }

    fn yy_at_eol(&self) -> bool {
        self.yytext.ends_with('\n')
    }

    fn yy_if_no_match(&mut self, _last_pos: usize) {
        if !self.buffer.is_empty() {
            let ch = self.buffer[0] as char;
            self.yy_at_bol = ch == '\n';
            print!("{ch}");
        }
        if !self.buffer.is_empty() {
            self.buffer.remove(0);
            if self.buffer_index > 0 {
                self.buffer_index -= 1;
            }
        }
    }
