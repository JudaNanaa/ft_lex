#[derive(Clone, Copy)]
pub struct AcceptElem {
    pub action: usize,
    pub dfa_state: usize,
    pub len_match: usize,
}

#[derive(Clone, Copy)]
pub enum NxtTable {
    Flat {
        cols: usize,
        flat: &'static [usize],
    },
    Packed {
        base: &'static [usize],
        nxt: &'static [usize],
        chk: &'static [usize],
    },
}

#[derive(Clone, Copy)]
pub struct Tables {
    pub nxt: NxtTable,
    pub yy_has_trans: &'static [u8],
    pub yy_ec: &'static [u8],
    pub yy_accept: &'static [u8],
    pub yy_trailing: &'static [u8],
    pub yy_trailing_accept: &'static [u8],
    pub yy_accept_cols: usize,
    pub yy_accept_flat: &'static [usize],
}

impl Tables {
    pub fn yy_finish_state(&self, state: usize) -> bool {
        self.yy_has_trans[state] != 0
    }
}

pub struct LexerCore<R: std::io::Read> {
    pub input: R,
    pub yytext: String,
    pub yyleng: usize,
    pub yy_start: usize,
    pub clean_flag: bool,
    pub yymore_flag: bool,
    pub yy_trailing_len: usize,
    pub yy_at_bol: bool,
    pub yy_init: bool,
    pub buffer: Vec<u8>,
    pub buffer_index: usize,
    pub stack: Vec<AcceptElem>,
}

impl<R: std::io::Read> LexerCore<R> {
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

    pub fn yy_next_char(&mut self) -> Option<u8> {
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

    pub fn yy_search_final(&mut self, state: usize, len_match: usize, tables: &Tables) {
        let mut i = 0;
        while tables.yy_accept_flat[state * tables.yy_accept_cols + i] != 0 {
            self.stack.push(AcceptElem {
                action: tables.yy_accept_flat[state * tables.yy_accept_cols + i],
                dfa_state: state,
                len_match,
            });
            i += 1;
        }
    }

    pub fn yy_pop_accepting_state(&mut self) -> AcceptElem {
        self.stack.pop().expect("pop on empty accepting stack")
    }

    pub fn yy_set_yytext(&mut self, elem: &AcceptElem) {
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

    pub fn yy_if_match_inner(&mut self, tables: &Tables) -> Option<usize> {
        let matching = self.yy_pop_accepting_state();
        if self.yy_trailing_len > 0 && tables.yy_trailing_accept[matching.dfa_state] != 0 {
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
        Some(matching.action)
    }

    pub fn yy_commit_match(&mut self) {
        if !self.clean_flag {
            let remaining = self.buffer[self.yyleng..].to_vec();
            self.buffer = remaining;
            self.buffer_index = 0;
            self.stack.clear();
            self.clean_flag = true;
        }
    }

    pub fn yy_at_eol(&self) -> bool {
        self.yytext.ends_with('\n') || self.buffer.get(self.yyleng) == Some(&b'\n')
    }

    pub fn yy_if_no_match(&mut self, _last_pos: usize) {
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
}

pub enum ActionResult {
    GuardFailed,
    Continue,
    Token(i32),
}

pub trait LexerInterface {
    type Reader: std::io::Read;
    fn get_core(&self) -> &LexerCore<Self::Reader>;
    fn get_core_mut(&mut self) -> &mut LexerCore<Self::Reader>;
    fn tables(&self) -> Tables;
    fn yy_is_exclusive_state(&self, state: usize) -> bool;
    fn yy_action(&mut self, action: usize) -> ActionResult;
}

fn dispatch_stack<L: LexerInterface>(lexer: &mut L, tables: &Tables) -> Option<i32> {
    while !lexer.get_core().stack.is_empty() {
        let action = {
            lexer.get_core_mut().yy_if_match_inner(tables).unwrap()
        };
        let new_at_bol = lexer.get_core().yytext.ends_with('\n');
        match lexer.yy_action(action) {
            ActionResult::GuardFailed => {
                lexer.get_core_mut().yy_at_bol = new_at_bol;
            }
            ActionResult::Continue => {
                lexer.get_core_mut().yy_commit_match();
                lexer.get_core_mut().yy_at_bol = new_at_bol;
                return None;
            }
            ActionResult::Token(v) => {
                lexer.get_core_mut().yy_commit_match();
                lexer.get_core_mut().yy_at_bol = new_at_bol;
                return Some(v);
            }
        }
    }
    lexer.get_core_mut().yy_commit_match();
    None
}

pub fn run_yylex<L: LexerInterface>(lexer: &mut L) -> i32 {
    if !lexer.get_core().yy_init {
        lexer.get_core_mut().yy_init = true;
    }

    let tables = lexer.tables();
    let mut len_match: usize = 0;
    let mut current_state: usize = lexer.get_core().yy_start;
    let mut last_accepting_state: usize = current_state;
    let mut last_accepting_pos: usize = 0;

    loop {
        let next_char = lexer.get_core_mut().yy_next_char();
        match next_char {
            None => {
                if last_accepting_state == 0 {
                    lexer.get_core_mut().yy_if_no_match(last_accepting_pos);
                } else if let Some(v) = dispatch_stack(lexer, &tables) {
                    return v;
                }
                break;
            }
            Some(c) => {
                len_match += 1;
                let yy_c = tables.yy_ec[c as usize] as usize;
                let next_state = match tables.nxt {
                    NxtTable::Flat { cols, flat } => flat[current_state * cols + yy_c],
                    NxtTable::Packed { base, nxt, chk } => {
                        let pos = base[current_state] + yy_c;
                        if chk[pos] == current_state { nxt[pos] } else { 0 }
                    }
                };

                {
                    let core = lexer.get_core_mut();
                    if tables.yy_trailing[next_state] != 0 {
                        core.yy_trailing_len = len_match;
                    }
                    if tables.yy_accept[next_state] != 0 {
                        core.yy_search_final(next_state, len_match, &tables);
                        last_accepting_state = next_state;
                        last_accepting_pos = core.buffer_index - 1;
                    }
                }

                if next_state == 0 || !tables.yy_finish_state(next_state) {
                    if last_accepting_state == 0 {
                        lexer.get_core_mut().yy_if_no_match(last_accepting_pos);
                    } else if let Some(v) = dispatch_stack(lexer, &tables) {
                        return v;
                    }
                    last_accepting_pos = 0;
                    last_accepting_state = 0;
                    current_state = lexer.get_core().yy_start;
                    len_match = 0;
                    lexer.get_core_mut().yy_trailing_len = 0;
                    lexer.get_core_mut().clean_flag = false;
                } else {
                    current_state = next_state;
                }
            }
        }
    }

    {
        let core = lexer.get_core_mut();
        core.stack.clear();
        core.yytext.clear();
        core.yyleng = 0;
        core.buffer.clear();
        core.buffer_index = 0;
    }
    0
}
