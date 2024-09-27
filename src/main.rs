use std::{
    fs,
    env,
    path::Path,
};

fn print_files(dir: &str, depth: usize) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let md = fs::metadata(&path).unwrap();

                        // Get the file or directory name
                        let name = entry.file_name();
                        let name = name.to_string_lossy(); // Convert OsString to String

                        // Indent based on depth
                        let indent = " ".repeat(depth * 4);

                        if md.is_dir() {
                            println!("{}Directory: {}", indent, name);
                            if let Some(path_str) = path.to_str() {
                                print_files(path_str, depth + 1); // Recurse into subdirectory
                            }
                        } else {
                            println!("{}File: {}", indent, name);
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Error reading directory {}: {}", dir, e),
    }
}

fn main() {
    // Get DIR Args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return;
    }

    let dir = &args[1];
    print_files(dir, 0); // Start at depth 0
}

