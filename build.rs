use comrak::nodes::{Ast, AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, Options};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::{format, Write as _};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::os;
use std::path::{Component, PathBuf};
use tiny_keccak::{Hasher, Sha3};

const SOURCE: &str = "src/parser/src.lalrpop";
const TARGET: &str = "src/parser/src.rs";

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed={SOURCE}");

    try_lalrpop(SOURCE, TARGET)?;

    try_write_down_versions_info()?;
    try_make_books()?;
    try_write_down_versions_info()?;
    try_make_books()?;
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

fn try_write_down_versions_info() -> std::result::Result<(), std::io::Error> {
    // get git commit hash
    let commit_hash = std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute git")
        .stdout;

    // get version or tag
    let version = std::process::Command::new("git")
        .args(&["describe", "--tags", "--always"])
        .output()
        .expect("failed to execute git")
        .stdout;
    // write to file
    let mut file = File::create("versions.txt")?;
    file.write_all(b"commit: ")?;
    file.write_all(&commit_hash)?;
    file.write_all(b"version: ")?;
    file.write_all(&version)?;
    Ok(())
}

fn try_make_books() -> std::result::Result<(), std::io::Error> {
    // open index.html from target/doc/*/index.html
    // CARGO_BUILD_TARGET_DIR
    let target_dir = "target";
    let docs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(target_dir)
        .join("doc");

    let mut org_docs = [
        "0intro.md",
        "language/0intro.md",
        "examples.md",
        "playground/index.md",
        "skill-tree.md",
        "research.md",
        "crates/index.md",
    ]
    .map(|s| PathBuf::from(s))
    .to_vec();

    let mut docs: Vec<_> =
        glob::glob(format!("{}/**/*.html", docs_path.as_path().to_string_lossy()).as_str())
            .expect("Failed to read glob pattern")
            .map(|entry| entry.unwrap())
            .collect();
    docs.sort_unstable();
    let mut docs = docs.iter().flat_map(|entry| {
        let full_path = entry.as_path();
        let last = full_path.with_extension("md");

        if last.ends_with("all.md")
        // || full_path.ends_with("index.md")
        {
            return None;
        }

        // make it relative to docs_path
        let last = last.strip_prefix(docs_path.clone()).unwrap();

        let base_name: Vec<_> = last.components().map(|c| c.as_os_str()).collect();
        let base_name = base_name[..base_name.len() - 1]
            .into_iter()
            .flat_map(|c| c.to_str())
            .collect::<Vec<&str>>()
            .join("/");

        if !base_name.as_str().starts_with("srclang") {
            return None;
        }
        let outpath = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("docs/crates/")
            .join(last);

        let target_base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("docs/crates/")
            .join(base_name.clone());

        // if basename doesn't exist mkdir -p it into existence
        if fs::read_dir(target_base_path.clone()).is_err() {
            fs::create_dir_all(target_base_path.clone()).expect("mkdir -p");
        }

        let html =
            File::open(full_path).expect(format!("index.html not found: {:?}", full_path).as_str());
        let reso = rustdoc_to_markdown::convert_rustdoc_to_markdown(html);
        let reso = reso.unwrap();
        let md = reso.0;
        // lets do some simple replacements.
        // we should have here a set of transformers that are fn(&mut str)

        std::fs::write(outpath.clone(), md).expect("book");

        let rel_path: PathBuf = outpath
            .strip_prefix(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/"))
            .unwrap()
            .into();
        Some(rel_path)
    });
    let mut docs = docs.collect::<Vec<_>>();
    docs.sort_unstable();
    docs.sort_by(|a, b| {
        // strip index.md from the end for both a and b
        let a = a
            .to_str()
            .unwrap_or("")
            .strip_suffix("index.md")
            .unwrap_or(a.to_str().unwrap());
        let b = b
            .to_str()
            .unwrap_or("")
            .strip_suffix("index.md")
            .unwrap_or(b.to_str().unwrap());
        a.cmp(b)
    });

    // org_docs.append(&mut docs);
    // let mut docs = org_docs;
    // docs.sort_unstable();
    let mut file = File::create("docs/SUMMARY.md")?;

    let write_docs = |docs: Vec<PathBuf>, mut file: File, indent: usize| -> Option<File> {
        let docs = docs.into_iter().map(|doc| PathBuf::from(doc));

        for doc in docs {
            let components = doc.components();
            let count = components.clone().count();
            // first print components.len() - 1 tabs
            // then print the last component
            let a = components.collect::<Vec<_>>();
            let a = a[0..count].iter().map(|c| c.as_os_str().to_str().unwrap());
            let name = a.map(|f| f.to_string()).collect::<Vec<String>>();

            let full_path =
                env!("CARGO_MANIFEST_DIR").to_string() + "/docs/" + doc.to_str().unwrap();

            let mut title = get_title(full_path.as_str());
            if !full_path.ends_with("playground/index.md") && title.is_empty() {
                println!("title is empty for: {:?}", full_path);
                continue;
            }
            if full_path.ends_with("playground/index.md") {
                title = "&#xec2b; srclang ide".to_string();
            }

            for c in name.iter().enumerate() {
                let header_level = c.0;
                for _ in 3..header_level {
                    // write!(file, "\t")?;
                }
                if c.0 == name.len() - 1 {
                    break;
                }
                // write!(
                //     file,
                //     "- [{}](#?{})\n",
                //     title[count - 1],
                //     title[0..count - 1].join("/")
                // )?;
            }
            let title_splat = title.split(" ");
            let firstbit = title_splat.clone().next().unwrap();
            let lastbit = title_splat.clone().last().unwrap();
            let lastbit_splat = lastbit.split("::");
            let lastbit_count = lastbit_splat.clone().count();
            for _ in 0..lastbit_count - 1 + indent {
                write!(file, "\t").unwrap_or(())
            }
            let ti = if lastbit_count == 1 {
                title.clone()
            } else {
                lastbit_splat.last().unwrap().to_string()
            };
            let icon = match firstbit {
                "Struct" => "&#xea91;",
                "Enum" => "&#xea95;",
                "Trait" => "&#xeb61;",
                "Function" => "&#xea8c;",
                "Type" => "&#xea92;",
                "Macro" => "&#xeb66;",
                "Constant" => "&#xeb5d;",
                "Module" => "&#xea8b;",
                "Crate" => "&#xeb29;",
                "Crates" => "🦀",
                _ => "",
            };
            writeln!(file, "- [{} {}]({})", icon, ti, doc.to_str().unwrap()).unwrap_or(());
        }
        Some(file)
    };

    write_docs(docs, write_docs(org_docs, file, 0).unwrap(), 1).unwrap();
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ItemPath {
    components: Vec<String>,
}

impl PartialOrd for ItemPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        eprintln!("partial_cmp: {:?} {:?}", self, other);
        for (a, b) in self.components.iter().zip(other.components.iter()) {
            match a.cmp(b) {
                Ordering::Equal => {
                    continue;
                }
                x => return Some(x),
            }
        }
        None
    }
}

impl Ord for ItemPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn parsed_title(path: &str) -> Option<ItemPath> {
    let title = get_title(path);
    let title = title.split(" ").last().unwrap().split("::");
    let components: Vec<_> = title.map(|c| c.to_string()).collect();
    // filter out empty strings
    let components: Vec<_> = components.into_iter().filter(|c| !c.is_empty()).collect();
    if components.is_empty() {
        return None;
    }
    Some(ItemPath { components })
}

fn parse_title(contents: &str) -> String {
    let arena = Arena::new();
    let root = parse_document(&arena, &contents, &Options::default());
    let mut title = String::new();
    for node in root.descendants() {
        match node.data.borrow().value {
            NodeValue::Heading(_) => {
                let mut txt_buf = String::new();
                for text_node in node.descendants() {
                    match text_node.data.borrow().value {
                        NodeValue::Text(ref text) => {
                            txt_buf.push_str(text);
                        }
                        _ => {}
                    }
                }
                title = txt_buf;
                break;
            }
            _ => {}
        }
    }
    title
}

fn get_title(path: &str) -> String {
    let file = File::open(path).expect(format!("file not found: {:?}", path).as_str());
    // read all the file
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("read failed");
    parse_title(&contents)
}
