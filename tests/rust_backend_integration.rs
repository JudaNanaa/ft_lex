use std::io::Write;
use std::process::Command;

fn run_ft_lex_rust(lex_src: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ft_lex"))
        .args(["--rust", "-t", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn ft_lex");
    child.stdin.take().unwrap().write_all(lex_src.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn generated_rust_compiles() {
    let lex_src = include_str!("fixtures/simple.lex");
    let generated = run_ft_lex_rust(lex_src);

    let dir = std::env::temp_dir();
    let rs_path = dir.join("ft_lex_test_output.rs");
    std::fs::write(&rs_path, &generated).unwrap();

    let status = Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", rs_path.to_str().unwrap()])
        .output()
        .expect("rustc not found");

    if !status.status.success() {
        eprintln!("--- Generated Rust ---\n{generated}");
        eprintln!("--- rustc stderr ---\n{}", String::from_utf8_lossy(&status.stderr));
        panic!("Generated Rust did not compile");
    }
}

#[test]
fn generated_rust_contains_lexer_struct() {
    let lex_src = include_str!("fixtures/simple.lex");
    let generated = run_ft_lex_rust(lex_src);
    assert!(generated.contains("pub struct Lexer<R: std::io::Read>"),
        "Missing Lexer struct in generated output");
    assert!(generated.contains("pub fn yylex("),
        "Missing yylex method in generated output");
    assert!(generated.contains("static YY_NXT_FLAT"),
        "Missing YY_NXT_FLAT table in generated output");
}
