
all: run

run:
	cargo r -- lex_files/test.l
	dot -Tpng dfa.dot -o dfa.png

test:
	cargo t

fmt:
	cargo fmt

clean:
	cargo clean

.PHONY: all run test fmt