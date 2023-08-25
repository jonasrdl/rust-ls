use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};
use std::ffi::CStr;
use libc;
use std::os::unix::fs::PermissionsExt;

use crate::output_format;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn list_files(path: &PathBuf, all: bool, long: bool) -> Result<()> {
    let entries = fs::read_dir(path)?;

    let filtered_entries: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().to_string();

            if !all && file_name_str.starts_with(".") {
                return None;
            }

            Some(entry)
        })
        .collect();

    if long {
        for entry in &filtered_entries {
            print_long_format(&entry)?;
        }
    } else {
        output_format::print_normal_format(filtered_entries)?;
    }

    Ok(())
}

fn print_long_format(entry: &fs::DirEntry) -> Result<()> {
    let metadata = entry.metadata()?;
    let file_name = entry.file_name();
    let file_name_str = file_name.to_string_lossy();

    let permission = format_permissions(metadata.permissions().mode());
    let owner = get_user_by_uid(metadata.uid()).unwrap_or("".to_string());
    let group = get_group_by_gid(metadata.gid()).unwrap_or("".to_string());
    let size = format_size(metadata.len());
    let mod_time: DateTime<Local> = DateTime::from(metadata.modified()?);
    let formatted_mod_time = mod_time.format("%b %e %H:%M").to_string();

    let file_type = if metadata.is_dir() {
        output_format::colorize_string(&file_name_str, "\x1B[34m")
    } else {
        file_name_str.to_string()
    };

    println!(
        "{:<10} {:<8} {:<8} {:<10} {:<15} {}",
        permission, owner, group, size, formatted_mod_time, file_type
    );

    Ok(())
}

fn get_user_by_uid(uid: u32) -> Option<String> {
    let passwd = unsafe { libc::getpwuid(uid) };
    if !passwd.is_null() {
        let cstr = unsafe { CStr::from_ptr((*passwd).pw_name) };
        return Some(cstr.to_string_lossy().to_string());
    }
    None
}

fn get_group_by_gid(gid: u32) -> Option<String> {
    let group = unsafe { libc::getgrgid(gid) };
    if !group.is_null() {
        let cstr = unsafe { CStr::from_ptr((*group).gr_name) };
        return Some(cstr.to_string_lossy().to_string());
    }
    None
}

fn format_size(size: u64) -> String {
    const UNIT: u64 = 1024;
    if size < UNIT {
        format!("{} B", size)
    } else {
        let exp = (size as f64).log(UNIT as f64).floor() as usize;
        let size_str = (size as f64 / UNIT.pow(exp as u32) as f64).to_string();
        let unit = match exp {
            1 => "K",
            2 => "M",
            3 => "G",
            4 => "T",
            _ => "P",
        };
        format!("{:.1} {}B", size_str, unit)
    }
}

fn format_permissions(mode: u32) -> String {
    let permissions = [
        (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
        (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
        (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
    ];

    let mut permission_string = String::with_capacity(9);

    for &(mask, perm) in &permissions {
        if mode & mask != 0 {
            permission_string.push(perm);
        } else {
            permission_string.push('-');
        }
    }

    permission_string
}
