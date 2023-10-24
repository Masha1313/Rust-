use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = fs::read_dir(&args[1]);

    if let Err(why) = result {
        println!("! {}", why.kind());
    } else if let Ok(paths) = result {
        for path in paths {
            if let Ok(entry) = path {
                let name = entry.file_name();
                println!("> {}", name.to_string_lossy());
            }
        }
    }
}