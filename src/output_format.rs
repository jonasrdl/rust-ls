use std::os::unix::fs::PermissionsExt;
use crate::entry_processing;
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
    let metadata = entry.metadata().unwrap();
    let file_name = entry.file_name();
    let file_name_str = file_name.to_string_lossy();

    if metadata.is_dir() {
        colorize_string(&bold(&file_name_str), "\x1B[34m")
    } else if metadata.permissions().mode() & 0o111 != 0 {
        colorize_string(&bold(&file_name_str), "\x1B[32;1m") // Green and bold
    } else if entry_processing::is_symlink(entry) {
        format!("{} -> {}", file_name_str, entry_processing::get_symlink_target(entry).unwrap_or_default())
    } else {
        file_name_str.to_string()
    }
}

pub fn print_normal_format_grouped(entries: Vec<fs::DirEntry>) -> Result<()> {
    let num_columns = 5;
    let column_spacing = 2; // Adjust this value for spacing between columns

    if entries.is_empty() {
        return Ok(());
    }

    let max_filename_length = entries.iter()
        .map(|entry| format_name(entry).len())
        .max()
        .unwrap_or(0);

    for (i, entry) in entries.iter().enumerate() {
        let filename = format_name(entry);
        print!("{:<width$}", filename, width = max_filename_length + column_spacing);

        if (i + 1) % num_columns == 0 {
            println!(); // Move to the next line after each row
        }
    }

    if entries.len() % num_columns != 0 {
        println!();
    }

    Ok(())
}

pub fn colorize_string(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, "\x1B[0m")
}

pub(crate) fn bold(text: &str) -> String {
    format!("\x1B[1m{}\x1B[0m", text)
}