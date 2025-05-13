
all: run

run:
	cargo r -- lex_files/test.l

dotfile:
	cargo run --features dotfile -- lex_files/test.l

test:
	cargo t

fmt:
	cargo fmt

clean:
	cargo clean

.PHONY: all run test fmt