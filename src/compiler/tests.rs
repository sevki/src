use okstd::prelude::*;

use super::*;

#[okstd::log(off)]
#[okstd::test]
fn debug() {
    let src = r#"use { native_fs, native_exec } from host
use { fs } from std

struct Innitguv {
    fs: native_fs,
    exec: native_exec
    current_pid: i32
}
"#;
    let db = &db::Database::default().enable_logging();

    let prog = SourceProgram::new(db, src.to_string(), "test".to_string());
    let res = super::compile(db, prog);
    println!("{:?}", prog);
    println!("{:?}", res.symbols(db));
    let modul = res.modul(db);
    println!("{:?}", modul);
}
