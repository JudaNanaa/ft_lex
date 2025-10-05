# ft_lex

A lexical analyzer generator written in Rust, inspired by the classic `lex` and `flex` tools.

## Overview

`ft_lex` is a lexical analyzer generator that reads specification files (`.l` files) containing regular expressions and actions, then generates C code to tokenize input according to those specifications. It transforms NFAs (Non-deterministic Finite Automata) to DFAs (Deterministic Finite Automata) for efficient pattern matching.

## Features

- **Regular Expression Support**: Full regex support including character classes, quantifiers, alternation, and grouping
- **Start Conditions**: Both inclusive (`%s`) and exclusive (`%x`) start conditions for managing lexer states
- **DFA Generation**: Automatic conversion from NFA to optimized DFA
- **Standard Lex Compatibility**: Supports common lex/flex syntax and features
- **C Code Generation**: Generates standalone C lexer code with minimal dependencies

## Installation

### Prerequisites

- Rust toolchain (stable)
- C compiler (gcc/clang)
- Make

### Building

```bash
# Build the project
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt
```

## Usage

### Basic Usage

```bash
# Generate lexer from specification
cargo run -- lex_files/test.l

# Compile the generated lexer
make
```

### Specification File Format

A `.l` file consists of three sections separated by `%%`:

```lex
%{
/* C code and includes */
#include <stdio.h>
%}

/* Definitions section */
%s COMMENT
%x STRING

%%

/* Rules section */
"/*"            BEGIN(COMMENT);
<COMMENT>"*/"   BEGIN(INITIAL);
[a-zA-Z_][a-zA-Z0-9_]*  printf("IDENTIFIER: %s\n", yytext);
[0-9]+          printf("NUMBER: %s\n", yytext);

%%

/* User code section */
int main(void) {
    yylex();
    return 0;
}
```

## Project Structure

```
ft_lex/
├── src/
│   ├── main.rs                 # Entry point
│   ├── file_parsing/           # Lex file parsing
│   │   ├── definitions/        # Definition section parsing
│   │   ├── rules/              # Rules section parsing
│   │   └── user_routine/       # User code section parsing
│   ├── regex/                  # Regex engine
│   │   ├── nfa/                # NFA construction
│   │   ├── dfa/                # DFA construction
│   │   └── tokenizer/          # Regex tokenization
│   └── lex_creation/           # C code generation
│       ├── functions/          # Generated helper functions
│       ├── tables/             # DFA transition tables
│       └── templates/          # C code templates
├── lex_files/                  # Example lexer specifications
└── Makefile                    # Build automation
```

## Features in Detail

### Regular Expressions

- **Character classes**: `[a-z]`, `[^0-9]`, `[:alpha:]`
- **Quantifiers**: `*`, `+`, `?`, `{n}`, `{n,}`, `{n,m}`
- **Alternation**: `a|b`
- **Grouping**: `(ab)+`
- **Escape sequences**: `\n`, `\t`, `\x41`, `\101`
- **Dot operator**: `.` (matches any character except newline)

### Start Conditions

```lex
%s INCLUSIVE_STATE
%x EXCLUSIVE_STATE

<STATE>pattern    action
<STATE1,STATE2>pattern    action
<*>pattern        action
```

### Built-in Functions

- `yytext`: Matched text
- `yyleng`: Length of matched text
- `BEGIN(state)`: Change lexer state
- `REJECT`: Re-match with next rule
- `yymore()`: Append next match to current
- `yyless(n)`: Return characters to input
- `input()`: Read one character
- `unput(c)`: Push character back

## Examples

The `lex_files/valid/` directory contains several example lexer specifications:

- `c_keywords.l`: C language keyword tokenizer
- `json.l`: JSON tokenizer
- `html.l`: HTML tag parser
- `minishell.l`: Shell command lexer
- `pascal.l`: Pascal language lexer

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with single thread (for file system tests)
cargo test -- --test-threads=1
```

### Generate DFA Visualization

```bash
# Build with dotfile feature
cargo run --features dotfile -- lex_files/test.l

# This generates dfa.dot and dfa.png files
```

### CI/CD

The project uses GitHub Actions for continuous integration:
- Code formatting checks (`cargo fmt`)
- Test execution
- Release binary builds

## Known Limitations

- Trailing context (`/`) is tokenized but not fully implemented
- Line/column number tracking for `^` and `$` anchors is incomplete
- Some advanced flex features are not yet supported

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `make fmt` and `make test`
5. Submit a pull request

## License

This project is part of the 42 school curriculum.

## References

- [Flex Manual](https://westes.github.io/flex/manual/)
- [Lex & Yacc](http://dinosaur.compilertools.net/)
- Dragon Book: Compilers: Principles, Techniques, and Tools
