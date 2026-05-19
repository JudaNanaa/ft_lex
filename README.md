# ft_lex

A lexical analyzer generator written in Rust, inspired by the classic `lex` and `flex` tools.

## Overview

`ft_lex` is a lexical analyzer generator that reads specification files (`.l` files) containing regular expressions and actions, then generates C code to tokenize input according to those specifications. It transforms NFAs (Non-deterministic Finite Automata) to DFAs (Deterministic Finite Automata) using equivalence classes for efficient pattern matching.

## Features

- **Regular Expression Support**: Full regex support including character classes, quantifiers, alternation, grouping, and trailing context
- **Start Conditions**: Both inclusive (`%s`) and exclusive (`%x`) start conditions for managing lexer states
- **Anchors**: BOL (`^`) and EOL (`$`) anchors
- **Trailing Context**: `pattern1/pattern2` syntax for lookahead matching
- **DFA Generation**: Automatic conversion from NFA to optimized DFA with equivalence class partitioning
- **Table Compression**: Prototype chaining compression (`-C`) to reduce generated table size
- **Standard Lex Compatibility**: Supports common lex/flex syntax and features
- **C Code Generation**: Generates standalone C lexer code with minimal dependencies
- **Rust Code Generation**: Optional Rust backend (`--rust`)
- **Multi-file Input**: Accepts multiple input files and/or stdin

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
# Generate lexer from specification file
cargo run -- lex_files/test.l

# Write output to stdout instead of ft_lex.yy.c
cargo run -- -t lex_files/test.l

# Read from stdin
cargo run -- -

# Compile the generated lexer
make
```

### Flags

| Flag | Description |
|------|-------------|
| `-t` | Write output to stdout instead of `ft_lex.yy.c` |
| `-v` | Print scanner statistics (NFA/DFA states, rules, equivalence classes) |
| `-n` | Suppress statistics output |
| `-C` | Enable table compression via prototype chaining |
| `--rust` | Generate Rust code instead of C (`ft_lex_yy.rs`) |

### Specification File Format

A `.l` file consists of three sections separated by `%%`:

```lex
%{
/* C code included verbatim at the top */
#include <stdio.h>
%}

/* Definitions: macros and start conditions */
DIGIT   [0-9]
ALPHA   [a-zA-Z]
%s COMMENT
%x STRING

%%

/* Rules section */
^hello          { printf("line starts with hello\n"); }
world$          { printf("line ends with world\n"); }
ab/cd           { printf("'ab' followed by 'cd', yytext=[%s]\n", yytext); }
{DIGIT}+        printf("NUMBER: %s\n", yytext);
{ALPHA}+        printf("WORD: %s\n", yytext);
"/*"            BEGIN(COMMENT);
<COMMENT>"*/"   BEGIN(INITIAL);
<*>.|\n         ;

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
│   ├── main.rs                  # Entry point, CLI argument parsing
│   ├── file_parsing/            # Lex file parsing
│   │   ├── definitions/         # Definition section parsing (macros, states, options)
│   │   ├── rules/               # Rules section parsing
│   │   ├── user_routine/        # User code section parsing
│   │   ├── combine.rs           # NFA combination and DFA construction
│   │   └── parsing.rs           # Top-level parse orchestration
│   ├── regex/                   # Regex engine
│   │   ├── nfa/                 # NFA construction from token stream
│   │   ├── dfa/                 # NFA→DFA subset construction
│   │   ├── tokenizer/           # Regex tokenization and postfix conversion
│   │   └── partition.rs         # Equivalence class partitioning
│   └── lex_creation/            # Code generation
│       ├── backend.rs           # Codegen backend trait
│       ├── c/                   # C backend (templates + writers)
│       ├── rust/                # Rust backend
│       ├── tables/              # DFA table generation (yy_nxt, yy_accept, yy_ec, ...)
│       └── stats.rs             # Scanner statistics
├── lex_files/                   # Example lexer specifications
└── Makefile                     # Build automation
```

## Features in Detail

### Regular Expressions

- **Character classes**: `[a-z]`, `[^0-9]`
- **POSIX classes**: `[:alnum:]`, `[:alpha:]`, `[:digit:]`, `[:lower:]`, `[:upper:]`, `[:xdigit:]`, `[:space:]`, `[:blank:]`, `[:cntrl:]`, `[:punct:]`, `[:print:]`, `[:graph:]`
- **Quantifiers**: `*`, `+`, `?`, `{n}`, `{n,}`, `{n,m}`
- **Alternation**: `a|b`
- **Grouping**: `(ab)+`
- **Escape sequences**: `\n`, `\t`, `\x41`, `\101`
- **Dot operator**: `.` (matches any character except newline)
- **Anchors**: `^` (beginning of line), `$` (end of line)
- **Trailing context**: `pattern/lookahead`
- **Macro expansion**: `{NAME}` expands a named definition

### Start Conditions

```lex
%s INCLUSIVE_STATE   /* active alongside INITIAL */
%x EXCLUSIVE_STATE   /* only active when explicitly selected */

<STATE>pattern         action
<STATE1,STATE2>pattern action
<*>pattern             action   /* active in all states */
```

### Options

```lex
%option array        /* yytext is a fixed-size char array (default size: 8192) */
%option pointer      /* yytext is a char* (default) */
%option yylmax=N     /* set array size to N when using %option array */
```

### Built-in Variables and Functions

| Name | Description |
|------|-------------|
| `yytext` | Matched text (pointer or array depending on `%option`) |
| `yyleng` | Length of matched text |
| `yyin` / `yyout` | Input / output FILE pointers (default: stdin / stdout) |
| `BEGIN(state)` | Switch to a start condition |
| `ECHO` | Write `yytext` to `yyout` |
| `REJECT` | Skip current match and try the next rule |
| `yymore()` | Append next match to current `yytext` |
| `yyless(n)` | Truncate `yytext` to `n` characters, returning the rest to input |
| `input()` | Read one character from input |
| `unput(c)` | Push character back into input |
| `yywrap()` | Called on EOF; return 1 to stop, 0 to continue |

### Generated Output

The C backend generates `ft_lex.yy.c`, a self-contained file including:

- DFA transition tables (`yy_nxt`, `yy_ec`, `yy_accept`, `yy_trailing`, ...)
- All runtime functions (`yylex`, `yymore`, `yyless`, `input`, `unput`, ...)
- The user's action code
- A weak `main` and `yywrap` that can be overridden

Compile with:

```bash
cc ft_lex.yy.c -o lexer
# or link against libl for the default yywrap/main:
cc ft_lex.yy.c -L. -ll -o lexer
```

## Examples

The `lex_files/valid/` directory contains several example lexer specifications:

- `c_keywords.l`: C language keyword tokenizer
- `json.l`: JSON tokenizer
- `html.l`: HTML tag parser
- `minishell.l`: Shell command lexer
- `pascal.l`: Pascal language lexer

`lex_files/test.l` exercises anchors and trailing context.

## Development

### Running Tests

```bash
cargo test

# Run with single thread (for file system tests)
cargo test -- --test-threads=1
```

### Generate DFA Visualization

```bash
cargo run --features dotfile -- lex_files/test.l
# Generates dfa.dot and dfa.png
```

### CI/CD

The project uses GitHub Actions for continuous integration:
- Code formatting checks (`cargo fmt`)
- Test execution
- Release binary builds

## References

- [Flex Manual](https://westes.github.io/flex/manual/)
- [Lex & Yacc](http://dinosaur.compilertools.net/)
- Dragon Book: Compilers: Principles, Techniques, and Tools
