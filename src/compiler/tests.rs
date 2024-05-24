#[cfg(test)]
#[okstd::test]
fn debug() {
    use salsa::{database::AsSalsaDatabase, storage::HasJarsDyn};

    use super::{db, text::SourceProgram};

    let src = r#"use { native_fs, native_exec } from host
use { fs } from std

struct Innitguv {
    fs: native_fs,
    exec: native_exec
    current_pid: i32
}

impl Exec for Innitguv {
    fn exec(&self, arg0: str, args: vec<str>) [nd, exec, await] -> i32 {
        let path = arg0
        let pid = self.exec.exec(path, args)
        if pid == -1 {
            raise(-1)
        }
        self.current_pid = pid
        yield()
    }
}

impl Actor for Innitguv {
    fn recv(&self, msg: Message) [recv, await] {
        self.exec(msg.path, msg.args)
    }
}"#;
    let db = &crate::compiler::db::Database::default().enable_logging();

    let prog = SourceProgram::new(db, src.to_string());
    let res = super::compile(db, prog);
    println!("{:?}", prog);
    println!("{:?}", res.symbols(db));
    let modul = res.modul(db);
    println!("{:?}", modul);
}
