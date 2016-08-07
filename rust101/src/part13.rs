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

