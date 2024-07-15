use core::panic;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tiny_keccak::{Hasher, Sha3};

const SOURCE: &str = "src/parser/src.lalrpop";
const TARGET: &str = "src/parser/src.rs";

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed={SOURCE}");

    try_lalrpop(SOURCE, TARGET)?;

    Ok(())
}

fn requires_lalrpop(source: &str, target: &str) -> bool {
    let target = if let Ok(target) = File::open(target) {
        target
    } else {
        println!("cargo:warning={TARGET} doesn't exist. regenerate.");
        return true;
    };

    let sha_prefix = "// sha3: ";
    let sha3_line = BufReader::with_capacity(128, target)
        .lines()
        .find_map(|line| {
            let line = line.unwrap();
            line.starts_with(sha_prefix).then_some(line)
        })
        .expect("no sha3 line?");
    let expected_sha3_str = sha3_line.strip_prefix(sha_prefix).unwrap();

    let actual_sha3 = {
        let mut hasher = Sha3::v256();
        let mut f = BufReader::new(File::open(source).unwrap());
        let mut line = String::new();
        while f.read_line(&mut line).unwrap() != 0 {
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            hasher.update(line.as_bytes());
            hasher.update(b"\n");
            line.clear();
        }
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        hash
    };
    let eq = sha_equal(expected_sha3_str, &actual_sha3);
    if !eq {
        println!("cargo:warning={TARGET} hash expected: {expected_sha3_str}");
        let mut actual_sha3_str = String::new();
        for byte in actual_sha3 {
            write!(actual_sha3_str, "{byte:02x}").unwrap();
        }
        println!("cargo:warning={TARGET} hash   actual: {actual_sha3_str}");
    }
    !eq
}

fn try_lalrpop(source: &str, target: &str) -> anyhow::Result<()> {
    if !requires_lalrpop(source, target) {
        return Ok(());
    }

    #[cfg(feature = "lalrpop")]
    {
        lalrpop::process_root().expect("running lalrpop failed");

        let full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(SOURCE);
        let path = full_path.to_str().unwrap();
        println!("cargo:rerun-if-changed={}", path);
        lalrpop::Configuration::new()
            .generate_in_source_tree()
            .process_file(path)
            .expect("msg");
        Ok(())
    }

    #[cfg(not(feature = "lalrpop"))]
    panic!("try: cargo build --manifest-path=compiler/parser/Cargo.toml --features=lalrpop");
}

fn sha_equal(expected_sha3_str: &str, actual_sha3: &[u8; 32]) -> bool {
    if expected_sha3_str.len() != 64 {
        panic!("lalrpop version? hash bug is fixed in 0.19.8");
    }

    let mut expected_sha3 = [0u8; 32];
    for (i, b) in expected_sha3.iter_mut().enumerate() {
        *b = u8::from_str_radix(&expected_sha3_str[i * 2..][..2], 16).unwrap();
    }
    *actual_sha3 == expected_sha3
}
