use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn run_ft_lex_rust(lex_src: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ft_lex"))
        .args(["--rust", "-t", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn ft_lex");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(lex_src.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "ft_lex exited with: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn run_ft_lex_c(lex_src: &str) -> String {
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_ft_lex"))
        .args(["-t", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn ft_lex");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(lex_src.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

fn build_runtime_rlib(dir: &std::path::Path) -> PathBuf {
    let runtime_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/lex_creation/rust/templates/ft_lex_runtime.rs");
    let rlib_path = dir.join("libft_lex_runtime.rlib");
    let status = Command::new("rustc")
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--crate-name",
            "ft_lex_runtime",
            runtime_src.to_str().unwrap(),
            "-o",
            rlib_path.to_str().unwrap(),
        ])
        .output()
        .expect("rustc not found");
    assert!(
        status.status.success(),
        "failed to compile ft_lex_runtime: {}",
        String::from_utf8_lossy(&status.stderr)
    );
    rlib_path
}

#[test]
fn rust_backend_generated_rust_compiles() {
    let lex_src = include_str!("fixtures/simple.lex");
    let generated = run_ft_lex_rust(lex_src);

    let dir = std::env::temp_dir();
    let rlib_path = build_runtime_rlib(&dir);
    let rs_path = dir.join("ft_lex_test_output.rs");
    std::fs::write(&rs_path, &generated).unwrap();

    let extern_arg = format!("ft_lex_runtime={}", rlib_path.to_str().unwrap());
    let status = Command::new("rustc")
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--extern",
            &extern_arg,
            rs_path.to_str().unwrap(),
        ])
        .output()
        .expect("rustc not found");

    if !status.status.success() {
        eprintln!("--- Generated Rust ---\n{generated}");
        eprintln!(
            "--- rustc stderr ---\n{}",
            String::from_utf8_lossy(&status.stderr)
        );
        panic!("Generated Rust did not compile");
    }
}

#[test]
fn rust_backend_generated_rust_contains_lexer_struct() {
    let lex_src = include_str!("fixtures/simple.lex");
    let generated = run_ft_lex_rust(lex_src);
    assert!(
        generated.contains("pub struct Lexer<R: std::io::Read>"),
        "Missing Lexer struct in generated output"
    );
    assert!(
        generated.contains("pub fn yylex("),
        "Missing yylex method in generated output"
    );
    assert!(
        generated.contains("static YY_NXT_FLAT"),
        "Missing YY_NXT_FLAT table in generated output"
    );
}

#[test]
fn c_backend_contains_yy_has_trans() {
    let lex_src = include_str!("fixtures/simple.lex");
    let generated = run_ft_lex_c(lex_src);
    assert!(
        generated.contains("yy_has_trans"),
        "Missing yy_has_trans in C output"
    );
}
