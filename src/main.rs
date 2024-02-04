use std::{env, fs, io::Write, io::Read};

trait FileFinder {
    fn find_file(&self, path: &str, file_to_find: &str, occurrences: &mut Vec<Occurrence>);
    fn display_files(&self, do_sort: bool, do_search: bool, file_list: &mut Vec<Occurrence>, text_to_find: &str, printer: &dyn Printer);
}

trait Printer {
    fn print(&self, occurrences: &mut Vec<Occurrence>);
}

struct DirSearcher;

struct ConsolePrinter;

struct FilePrinter {
    output_file: String,
}

enum Occurrence {
    File(String),
    Directory(String),
    TextFile(String),
    SearchableText(String),
}

fn find_text(path: &str, text_to_find: &str, text_list: &mut Vec<Occurrence>) -> Result<(), std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    for (i, line) in text.lines().enumerate() {
        if line.contains(text_to_find) {
            text_list.push(Occurrence::SearchableText(format!("In file <{}>: At line {}: {}", path, i + 1, line)));
        }
    }
    Ok(())
}

fn bubble_sort(occurrences: &mut Vec<Occurrence>) {
    let n = occurrences.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if occurrences[j].file_name() > occurrences[j + 1].file_name() {
                occurrences.swap(j, j + 1);
            }
        }
    }
}

impl FileFinder for DirSearcher {
    fn find_file(&self, path: &str, file_to_find: &str, file_list: &mut Vec<Occurrence>) {
        let result = fs::read_dir(path);
        if let Err(_) = result {
            eprintln!("Failed to open: {}", path);
        } else if let Ok(paths) = result {
            for path in paths {
                if let Ok(entry) = path {
                    let cur_path = entry.path();
                    let file_name = entry.file_name();

                    if cur_path.is_file() {
                        if !file_to_find.is_empty() {
                            if file_name == file_to_find {
                                if let Some(ext) = cur_path.extension().and_then(|s| s.to_str()) {
                                    if ext == "rs" || ext == "txt" {
                                        file_list.push(Occurrence::TextFile(cur_path.display().to_string()));
                                    } else {
                                        file_list.push(Occurrence::File(cur_path.display().to_string()));
                                    }
                                }
                            }
                        } else {
                            if let Some(ext) = cur_path.extension().and_then(|s| s.to_str()) {
                                if ext == "rs" || ext == "txt" {
                                    file_list.push(Occurrence::TextFile(cur_path.display().to_string()));
                                } else {
                                    file_list.push(Occurrence::File(cur_path.display().to_string()));
                                }
                            }
                        }
                    } else {
                        file_list.push(Occurrence::Directory(cur_path.display().to_string()));
                        self.find_file(cur_path.to_str().unwrap(), file_to_find, file_list);
                    }
                }
            }
        } else {
            eprintln!("Failed to open directory: {}", path);
            std::process::exit(1);
        }
    }

    fn display_files(&self, do_sort: bool, do_search: bool, file_list: &mut Vec<Occurrence>, text_to_find: &str, printer: &dyn Printer) {
        if do_search {
            let mut text_occurrences: Vec<Occurrence> = Vec::new();
            for file_name in file_list {
                if let Occurrence::TextFile(path) = file_name {
                    if let Err(e) = find_text(&path, text_to_find, &mut text_occurrences) {
                        eprintln!("Error reading file {}: {}", path, e);
                    }
                }
            }
            if do_sort {
                bubble_sort(&mut text_occurrences);
            }


            printer.print(&mut text_occurrences);
        } else {
            printer.print(file_list);
        }
    }
}

impl Printer for ConsolePrinter {
    fn print(&self, occurrences: &mut Vec<Occurrence>) {
        for occurrence in occurrences {
            match occurrence {
                Occurrence::File(path) | Occurrence::Directory(path) | Occurrence::TextFile(path) | Occurrence::SearchableText(path) => {
                    println!("{}", path);
                }
            }
        }
    }
}

impl Printer for FilePrinter {
    fn print(&self, occurrences: &mut Vec<Occurrence>) {
        if let Ok(mut file) = fs::OpenOptions::new().write(true).truncate(true).create(true).open(&self.output_file) {
            for occurrence in occurrences {
                match occurrence {
                    Occurrence::File(path) | Occurrence::Directory(path) | Occurrence::TextFile(path) | Occurrence::SearchableText(path) => {
                        writeln!(file, "{}", path).expect("Failed to write into a file");
                    }
                }
            }
            println!("Output written to '{}'", &self.output_file);
        }
    }
}

impl Occurrence {
    fn file_name(&self) -> &str {
        match self {
            Occurrence::File(path) | Occurrence::Directory(path) | Occurrence::TextFile(path) | Occurrence::SearchableText(path) => path
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        eprintln!("Usage: <path> [--find <file_name>] [--sort] [-f <output_file>] [--in-file]");
        return;
    }

    let search_path = &arguments[1];
    let mut file_to_find: Option<&str> = None;
    let mut text_te_find: Option<&str> = None;
    let mut in_file_search = false;
    let mut sort_flag = false;
    let mut occurrences = Vec::new();
    let mut print_strategy: Box<dyn Printer> = Box::new(ConsolePrinter);

    for i in 2..arguments.len() {
        match arguments[i].as_str() {
            "--find" if i + 1 < arguments.len() => {
                file_to_find = Some(&arguments[i + 1]);
            }
            "-f" if i + 1 < arguments.len() => {
                let output_file = Some(&arguments[i + 1]);
                print_strategy = Box::new(FilePrinter{output_file: output_file.unwrap().parse().unwrap() })
            }
            "--in-file" => {
                in_file_search = true;
                text_te_find = Some(&arguments[i + 1]);
            },
            "--sort" => sort_flag = true,
            _ => {}
        }
    }

    let dir_searcher = DirSearcher;

    dir_searcher.find_file(search_path, file_to_find.unwrap_or(""), &mut occurrences);
    dir_searcher.display_files(sort_flag, in_file_search, &mut occurrences, text_te_find.unwrap_or(""), &*print_strategy);
}
