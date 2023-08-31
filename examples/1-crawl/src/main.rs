use std::env;
use std::fs;
use std::path::Path;

fn walk_path(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        println!("Entry path: {}", entry.path().display());
        let path = entry.path();
        if path.is_dir() {
            walk_path(&path);
        } else {
            println!("{}", path.display());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        println!("Path {} does not exist", path.display());
        return;
    }

    walk_path(path);
}
