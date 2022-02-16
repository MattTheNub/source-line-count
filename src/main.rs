use crate::count::count;
use anyhow::Context;
use clap::Parser;
use std::{fs, path::PathBuf};

pub mod count;
pub mod lang;
pub mod util;

#[cfg(test)]
mod tests;

#[derive(Parser)]
struct Cli {
	#[clap(parse(from_os_str))]
	file: PathBuf,
}

fn main() -> anyhow::Result<()> {
	let args = Cli::parse();

	let file = fs::read_to_string(&args.file)
		.with_context(|| format!("Could not read file `{}`", args.file.to_string_lossy()))?;

	let ext = match args.file.extension() {
		Some(ext) => ext.to_str().unwrap_or(""),
		None => "",
	};
	let lines =
		count(&file, ext).with_context(|| format!("The extension '{}' is not supported", ext))?;

	println!("{}", lines);

	Ok(())
}
