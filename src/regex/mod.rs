pub mod tokenizer;
pub use tokenizer::*;
pub mod nfa;
pub use nfa::*;
pub mod dfa;
pub mod partition;
#[cfg(test)]
mod tests_regex;
mod utils;
