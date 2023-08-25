use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn print_normal_format(entries: Vec<fs::DirEntry>) -> Result<()> {
    let formatted_names: String = entries
        .iter()
        .map(|entry| format_name(entry))
        .collect::<Vec<_>>()
        .join("  ");

    println!("{}", formatted_names);

    Ok(())
}

fn format_name(entry: &fs::DirEntry) -> String {
    let metadata = entry.metadata().unwrap(); // Safe to unwrap because it's used within a map
    let file_name = entry.file_name();
    let file_name_str = file_name.to_string_lossy();

    if metadata.is_dir() {
        colorize_string(&bold(&file_name_str), "\x1B[34m")
    } else {
        file_name_str.to_string()
    }
}

pub fn colorize_string(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, "\x1B[0m")
}

fn bold(text: &str) -> String {
    format!("\x1B[1m{}\x1B[0m", text)
}