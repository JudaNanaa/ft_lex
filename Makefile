
all: run

run:
	cargo r -- lex_files/test.l
	cc -c src/lex_creation/templates/libl_functions.c -g3
	ar -rcs libl.a libl_functions.o

dotfile:
	cargo run --features dotfile -- lex_files/test.l

test:
	cargo t

fmt:
	cargo fmt

clean:
	cargo clean

.PHONY: all run test fmt