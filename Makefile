
all: run

run:
	cargo r -- lex_files/test.l
	mv ./target/debug/ft_lex .
	cc -c src/lex_creation/c/templates/libl_functions.c -g3
	ar -rcs libl.a libl_functions.o
	cc -Wall -Wextra -Werror ft_lex.yy.c -L. -ll -g3

libft_lex_runtime.rlib: src/lex_creation/rust/templates/ft_lex_runtime.rs
	rustc --edition=2021 --crate-type=lib --crate-name=ft_lex_runtime $< -o $@

run_rust: libft_lex_runtime.rlib
	cargo r -- --rust lex_files/test_rust.l
	rustc --edition=2021 ft_lex_yy.rs --extern ft_lex_runtime=./libft_lex_runtime.rlib -o ft_lex_yy

dotfile:
	cargo run --features dotfile -- lex_files/test.l

test:
	cargo t

fmt:
	cargo fmt

clean:
	cargo clean

fclean: clean
	rm -rf ft_lex.yy.c ft_lex_yy.rs lex.yy.c libl_functions.o libl.a libft_lex_runtime.rlib a.out ft_lex_yy ft_lex
	rm -rf target

push: fmt test fclean
	@echo now you can push the code

re: fclean all

.PHONY: all run run_rust test fmt clean fclean re push
