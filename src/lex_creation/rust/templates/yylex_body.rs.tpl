    pub fn yylex(&mut self) -> i32 {
        #write_in_yylex

        if !self.yy_init {
            self.yy_init = true;
        }

        let mut len_match: usize = 0;
        let mut current_state: usize = self.yy_start;
        let mut last_accepting_state: usize = self.yy_start;
        let mut last_accepting_pos: usize = 0;

        loop {
            match self.yy_next_char() {
                None => {
                    if last_accepting_state == 0 {
                        self.yy_if_no_match(last_accepting_pos);
                    } else {
                        if let Some(v) = self.yy_if_match() {
                            return v;
                        }
                    }
                    break;
                }
                Some(c) => {
                    len_match += 1;
                    let yy_c = YY_EC[c as usize] as usize;
                    let next_state = YY_NXT_FLAT[current_state * YY_NXT_COLS + yy_c] as usize;
                    if YY_TRAILING[next_state] != 0 {
                        self.yy_trailing_len = len_match;
                    }
                    if YY_ACCEPT[next_state] != 0 {
                        self.yy_search_final(next_state, len_match);
                        last_accepting_state = next_state;
                        last_accepting_pos = self.buffer_index - 1;
                    }
                    if next_state == 0 || !self.yy_finish_state(next_state) {
                        if last_accepting_state == 0 {
                            self.yy_if_no_match(last_accepting_pos);
                        } else {
                            if let Some(v) = self.yy_if_match() {
                                return v;
                            }
                        }
                        last_accepting_pos = 0;
                        last_accepting_state = 0;
                        current_state = 0;
                        len_match = 0;
                        self.yy_trailing_len = 0;
                        self.clean_flag = false;
                    }
                    current_state = next_state;
                }
            }
        }

        self.stack.clear();
        change_me_clear_yytext!
        self.buffer.clear();
        self.buffer_index = 0;
        0
    }
} // end impl<R: std::io::Read> Lexer<R>
