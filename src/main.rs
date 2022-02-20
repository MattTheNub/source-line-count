use crate::count::count;
use anyhow::{bail, Context};
use clap::Parser;
use std::{cmp, fs, path::PathBuf};

pub mod count;
pub mod lang;
pub mod util;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[clap(short, long)]
	recursive: bool,
	#[clap(required = true, parse(from_os_str))]
	files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let args = Cli::parse();

	let mut file_lines = Vec::new();
	let mut file_skips = 0;
	// If only one file is skipped, we will display the extension
	let mut skipped_ext = None;

	for path in args.files {
		handle_path(
			path,
			args.recursive,
			&mut file_lines,
			&mut skipped_ext,
			&mut file_skips,
		)?;
	}
	if file_lines.is_empty() {
		// If we haven't counted any files, there must have been an issue with the user's input
		match file_skips {
			0 => (),
			1 => {
				// When only one file was skipped, give specific information
				bail!("The extension '{}' is not supported", skipped_ext.unwrap());
			}
			2..=u32::MAX => {
				bail!("All {} files have unsupported extensions", file_skips);
			}
		}

		bail!("No files provided")
	}
	if file_skips > 0 {
		eprintln!(
			"Warning: {} files with invalid extensions ignored",
			file_skips
		);
	}

	// Make a table of line counts for all files

	// Find the width we need to use for each column
	let mut file_len = "File".len();
	let mut lines_len = "Lines".len();
	for (file, lines) in &file_lines {
		file_len = cmp::max(file_len, file.len());
		lines_len = cmp::max(lines_len, lines.to_string().len());
	}
	let row_len = file_len + lines_len + 5;

	// Print the header row
	print!(" {:width$} ", "File", width = file_len);
	print!("| {:width$} ", "Lines", width = lines_len);
	println!();
	// Separator
	println!("{}", "=".repeat(row_len));
	for (file, lines) in &file_lines {
		print!(" {:width$} ", file, width = file_len);
		print!("| {:width$} ", lines, width = lines_len);
		println!();
	}
	// Separator
	println!("{}", "=".repeat(row_len));
	print!(" {:width$} ", "Total", width = file_len);
	print!(
		"| {:width$} ",
		file_lines.iter().fold(0, |total, elem| total + elem.1),
		width = lines_len
	);
	println!();

	Ok(())
}

fn handle_path(
	path: PathBuf,
	recursive: bool,
	file_lines: &mut Vec<(String, usize)>,
	skipped_ext: &mut Option<String>,
	file_skips: &mut u32,
) -> anyhow::Result<()> {
	let dir = fs::read_dir(&path);
	if let Ok(dir) = dir {
		if recursive {
			for entry in dir.flatten() {
				handle_path(entry.path(), recursive, file_lines, skipped_ext, file_skips)?;
			}
		} else {
			eprintln!(
			"Warning: Skipping directory '{}' (hint: use the --recursive flag to traverse directories)",
			path.to_string_lossy()
		);
		}
		return Ok(());
	}
	let file = fs::read(&path)
		.with_context(|| format!("Could not read file `{}`", path.to_string_lossy()))?;
	let file = String::from_utf8_lossy(&file).to_string();

	let ext = match path.extension() {
		Some(ext) => ext.to_str().unwrap_or(""),
		None => "",
	};
	let lines = count(&file, ext);
	// .with_context(|| format!("The extension '{}' is not supported", ext))?;

	if let Some(lines) = lines {
		let path_string = path.to_string_lossy().to_string();
		file_lines.push((path_string, lines));
	} else {
		*skipped_ext = Some(ext.to_owned());
		*file_skips += 1;
	}

	Ok(())
}
