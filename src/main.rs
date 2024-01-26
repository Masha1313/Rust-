use std::{env, fs, io::Write};

trait ListPrinter {
    fn print(&self, file_list: &mut Vec<String>);
}

struct ConsolePrinter;

impl ListPrinter for ConsolePrinter {
    fn print(&self, file_list: &mut Vec<String>) {
        for file in file_list.iter() {
            println!("{}", file);
        }
    }
}

struct FileSaver {
    output_file: String,
}

impl ListPrinter for FileSaver {
    fn print(&self, file_list: &mut Vec<String>) {
        let mut output = match fs::File::create(&self.output_file) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file '{}': {}", &self.output_file, e);
                return;
            }
        };

        for file_name in file_list.iter() {
            if let Err(e) = writeln!(output, "{}", file_name) {
                eprintln!("Error writing to file '{}': {}", &self.output_file, e);
                return;
            }
        }

        println!("Output written to '{}'", &self.output_file);
    }
}

trait FileFinder {
    fn find_file(&self, path: &str, file_to_find: &str, file_list: &mut Vec<String>);
    fn display_files(&self, sort: bool, file_list: &mut Vec<String>, printer: &dyn ListPrinter);
}

struct DirSearcher;

fn bubble_sort(file_list: &mut Vec<String>) {
    let n = file_list.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if file_list[j] > file_list[j + 1] {
                file_list.swap(j, j + 1);
            }
        }
    }
}

impl FileFinder for DirSearcher {
    fn find_file(&self, path: &str, file_to_find: &str, file_list: &mut Vec<String>) {
        let result = fs::read_dir(path);
        if let Err(_) = result {
            eprintln!("Failed to open: {}", path);
        } else if let Ok(paths) = result {
            for path in paths {
                if let Ok(entry) = path {
                    let cur_path = entry.path();
                    let file_name = entry.file_name();

                    if cur_path.is_file() {
                        if !file_to_find.is_empty() && file_name.to_string_lossy() == file_to_find {
                            file_list.push(cur_path.display().to_string());
                        } else if file_to_find.is_empty() {
                            file_list.push(cur_path.display().to_string());
                        }
                    } else {
                        self.find_file(cur_path.to_str().unwrap(), file_to_find, file_list);
                    }
                }
            }
        }
    }

    fn display_files(&self, sort: bool, file_list: &mut Vec<String>, printer: &dyn ListPrinter) {
        if sort {
            bubble_sort(file_list);
        }
        printer.print(file_list);
    }
}

fn main() {
    let dir_searcher = DirSearcher;
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        eprintln!("Usage: <path> [--find <file_name>] [--sort] [-f <output_file>]");
        return;
    }

    let search_path = &arguments[1];
    let mut search_file = "";
    let mut enable_sort = false;
    let mut files_found: Vec<String> = Vec::new();

    let mut printer: Box<dyn ListPrinter> = Box::new(ConsolePrinter);

    for i in 2..arguments.len() {
        match arguments[i].as_str() {
            "--find" if i + 1 < arguments.len() => search_file = &arguments[i + 1],
            "--sort" => enable_sort = true,
            "-f" if i + 1 < arguments.len() => printer = Box::new(FileSaver { output_file: arguments[i + 1].clone() }),
            _ => {}
        }
    }

    dir_searcher.find_file(search_path, search_file, &mut files_found);
    dir_searcher.display_files(enable_sort, &mut files_found, &*printer);
}
