use std::fs;
use std::io::Write;

use fnv::FnvHashSet;
use ignore::WalkBuilder;
use phf::phf_set;
use walkdir::WalkDir;

static IGNORED_FILES: phf::Set<&'static str> = phf_set!(".gitignore", ".env");
const ROOT_PATH: &'static str = ".";

fn main() {
    let unignored_paths: FnvHashSet<_> = WalkBuilder::new(ROOT_PATH)
        .require_git(false)
        .build()
        .filter_map(Result::ok)
        .map(|entry| entry.path().as_os_str().to_owned())
        .collect();
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for entry in WalkDir::new(ROOT_PATH).contents_first(true) {
        match entry {
            Ok(entry) => {
                if !unignored_paths.contains(entry.path().as_os_str())
                    && !IGNORED_FILES.contains(&entry.file_name().to_string_lossy())
                {
                    writeln!(lock, "Deleting: {}", entry.path().display()).unwrap();
                    if entry.path().is_file() {
                        if fs::remove_file(entry.path()).is_err() {
                            writeln!(lock, "Failed to delete file {}", entry.path().display())
                                .unwrap();
                        }
                    } else if entry.path().is_dir() {
                        if fs::remove_dir(entry.path()).is_err() {
                            eprintln!("Failed to delete directory {}", entry.path().display());
                        }
                    }
                }
            }
            Err(_) => writeln!(lock, "Error accessing entry").unwrap(),
        }
    }
}
