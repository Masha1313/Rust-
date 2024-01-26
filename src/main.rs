use std::{env, fs};

trait FileFinder {
    fn find_file(&self, path: String, file_to_find: String);
}

struct DirSearcher;

impl FileFinder for DirSearcher {
    fn find_file(&self, path: String, file_to_find: String) {
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
                        self.find_file(cur_path.to_string_lossy().to_string(), file_to_find.clone())
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
        2 => file_searcher.find_file(args[1].clone(), String::new()),
        4 if args[2] == "--find" => file_searcher.find_file(args[1].clone(), args[3].clone()),
        _ => eprintln!("Usage: program_name <path> [--find <file_name>]"),
    }
}
