use std::io::prelude::*;
use std::{io, fs, thread};
use std::sync::Arc;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

enum OutputMode {
    Print,
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

fn filter_lines(options: Arc<Options>, in_channel: Receiver<String>, out_channel: SyncSender<String>) {
    for line in in_channel.iter() {
	if line.contains(&options.pattern) {
	    out_channel.send(line).unwrap();
	}
    }
}

fn do_output(options: Arc<Options>, in_channel: Receiver<String>) {
    match options.output_mode {
	OutputMode::Print => {
	    for line in in_channel.iter() {
		println!("{}", line);
	    }
	},
	OutputMode::Count => {
	    let count = in_channel.iter().count();
	    println!("{} matches for {}", count, options.pattern);
	},
    }
}

fn run(options: Options) {
    let aoptions = Arc::new(options);

    let (file_tx, file_rx) = sync_channel(16);
    let (filter_tx, filter_rx) = sync_channel(16);

    let file_options = aoptions.clone();
    let file_thread = thread::spawn(move || read_files(file_options, file_tx));
    let filter_options = aoptions.clone();
    let filter_thread = thread::spawn(move || filter_lines(filter_options, file_rx, filter_tx));
    let output_options = aoptions.clone();
    let output_thread = thread::spawn(move || do_output(output_options, filter_rx));

    file_thread.join().unwrap();
    filter_thread.join().unwrap();
    output_thread.join().unwrap();
}

fn main() {
    let files: Vec<String> = vec!["../src/part11.rs".to_string(), "../src/part12.rs".to_string()];
    let pattern: String = String::from("let mut");
    let options = Options::new(files, pattern, OutputMode::Count);
    run(options);

    let print_files: Vec<String> = vec!["../src/part11.rs".to_string(), "../src/part12.rs".to_string()];
    let print_pattern: String = String::from("let mut");
    let print_options = Options::new(print_files, print_pattern, OutputMode::Print);
    run(print_options);
}

#[cfg(test)]
mod tests {
    use std::{io, fs, thread};
    use std::sync::Arc;
    use std::sync::mpsc;
    use super::filter_lines;
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

    #[test]
    fn test_filter_lines() {
        let files: Vec<String> = vec![String::from("/tmp/test_file_one")];
        let pattern: String = String::from("pmatch");
        let output_mode = OutputMode::Count;
        let options = Arc::new(Options::new(files, pattern.clone(), output_mode));
        let (in_tx, in_rx) = mpsc::sync_channel(1);
        let (out_tx, out_rx) = mpsc::sync_channel(1);

        thread::spawn(move || filter_lines(options, in_rx, out_tx));

	let strings = vec!["foo", "boo", "asdf pmatchfoo", "pmatch"];
	for s in strings {
	    let string = String::from(s);
	    in_tx.send(string).unwrap();
	}
	drop(in_tx);
	let mut num_found = 0;
        for read_line in out_rx.iter() {
	    assert!(read_line.contains(&pattern));
	    num_found = num_found + 1;
        }
	assert_eq!(num_found, 2);
    }
}
