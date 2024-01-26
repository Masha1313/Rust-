use std::{env, fs};

trait FileFinder {
    fn find_file(&self, path: &str, file_to_find: &str);
}

struct DirSearcher;

impl FileFinder for DirSearcher {
    fn find_file(&self, path: &str, file_to_find: &str) {
        let result = fs::read_dir(path);
        if let Err(why) = result {
            println!("! {}", why.kind());
        } else if let Ok(paths) = result {
            for path in paths {
                if let Ok(entry) = path {
                    let file_name = entry.file_name();
                    let cur_path = entry.path();

                    if cur_path.is_file() {
                        if file_to_find.is_empty() {
                            println!("{}", file_name.to_string_lossy());
                        }
                        if file_name.to_string_lossy().to_string() == file_to_find {
                            println!("File location: {:?}", cur_path);
                        }
                    } else {
                        self.find_file(cur_path.to_str().unwrap(), file_to_find)
                    }
                }
            }
        }
    }
}

fn main() {
    let file_searcher = DirSearcher;
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => file_searcher.find_file(&args[1], ""),
        4 if args[2] == "--find" => file_searcher.find_file(&args[1], &args[3]),
        _ => eprintln!("Usage: program_name <path> [--find <file_name>]"),
    }
}
