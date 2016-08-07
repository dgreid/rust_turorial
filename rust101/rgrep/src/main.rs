use std::io::prelude::*;
use std::{io, fs, thread};
use std::sync::Arc;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

enum OutputMode {
    Print,
    SortAndPrint,
    Count,
}

struct Options {
    files: Vec<String>,
    pattern: String,
    output_mode: OutputMode,
}

impl Options {
    fn new(files: Vec<String>, pattern: String, output_mode: OutputMode) -> Self {
        Options {
            files: files,
            pattern: pattern,
            output_mode: output_mode,
        }
    }
}


fn read_files(options: Arc<Options>, out_channel: SyncSender<String>) {
    for file_name in options.files.iter() {
        let file = match fs::File::open(file_name) {
            Err(_) => {
                println!("Failed to read file {}.", file_name);
                return;
            },
            Ok(file) => {
                file
            },
        };

        let file_buf = io::BufReader::new(file);
        for line_res in file_buf.lines() {
            let line = line_res.unwrap();
            out_channel.send(line).unwrap();
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use std::{io, fs, thread};
    use std::sync::Arc;
    use std::sync::mpsc;
    use super::read_files;
    use super::OutputMode;
    use super::Options;

    #[test]
    fn test_read_files() {
        let files: Vec<String> = vec![String::from("/tmp/test_file_one")];
        let pattern: String = String::from("asdfasdf");
        let output_mode = OutputMode::Count;
        let options = Arc::new(Options::new(files, pattern, output_mode));
        let (tx, rx) = mpsc::sync_channel(1);

        thread::spawn(move || read_files(options, tx));
        for read_line in rx.iter() {
            println!("{}", read_line);
        }
    }
}
