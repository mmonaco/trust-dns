extern crate syntex;
extern crate rustlex_codegen;

use std::env;
use std::path::Path;

pub fn main() {
    let mut registry = syntex::Registry::new();
    rustlex_codegen::plugin_registrar(&mut registry);
    let src = Path::new("src/serialize/txt/master_lex.in.rs");
    let dst = Path::new(&env::var_os("OUT_DIR").unwrap()).join("master_lex.rs");
    registry.expand("", &src, &dst).unwrap();
}
