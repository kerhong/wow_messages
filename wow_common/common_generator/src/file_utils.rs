use std::fs::read_to_string;
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) fn workspace_directory() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(p.pop());
    assert!(p.pop());
    p
}

pub(crate) fn vanilla_dir() -> PathBuf {
    workspace_directory()
        .join("wow_common")
        .join("wow_common")
        .join("src")
        .join("vanilla")
}

pub(crate) fn tbc_dir() -> PathBuf {
    workspace_directory()
        .join("wow_common")
        .join("wow_common")
        .join("src")
        .join("tbc")
}

pub(crate) fn wrath_dir() -> PathBuf {
    workspace_directory()
        .join("wow_common")
        .join("wow_common")
        .join("src")
        .join("wrath")
}

pub(crate) fn write_string_to_file(s: &str, filename: &Path) {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename);
    let mut f = match f {
        Ok(f) => f,
        Err(_) => {
            let dir = filename.parent().unwrap();
            std::fs::create_dir_all(dir).unwrap();
            std::fs::File::create(filename)
                .unwrap_or_else(|_| panic!("unable to open file: '{}'", filename.to_str().unwrap()))
        }
    };

    f.write_all(s.as_bytes()).unwrap();
}

pub(crate) fn overwrite_if_not_same_contents(s: &str, filename: &Path) {
    let f = read_to_string(filename).unwrap();
    if f != s {
        write_string_to_file(s, filename);
    }
}

pub(crate) fn insert_between(contents: &str, start: &str, end: &str, replace_with: &str) -> String {
    let (before, mid) = contents.split_once(start).unwrap();
    let (_, after) = mid.split_once(end).unwrap();

    let mut s = before.to_string();
    s += start;
    s += "\n";
    s += replace_with;
    s += end;
    s += after;

    s
}

pub(crate) fn overwrite_autogenerate_if_not_the_same(path: &Path, s: &str) {
    let contents = match read_to_string(path) {
        Ok(e) => e,
        Err(e) => {
            panic!("File error in '{}' '{}'", path.display(), e);
        }
    };

    let s = insert_between(
        &contents,
        "// AUTOGENERATED_START",
        "// AUTOGENERATED_END",
        s,
    );
    overwrite_if_not_same_contents(&s, path);
}