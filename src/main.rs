use std::{fs, os::unix::fs::symlink, path::Path};

use clap::Parser;
use walkdir::{DirEntry, WalkDir};

const HWMON_DIR: &str = "/sys/class/hwmon";
const PATTERN_TEMP: &str = "temp";
const PATTERN_INPUT: &str = "_input";
const PATTERN_LABEL: &str = "_label";
const PATTERN_NAME: &str = "name";

#[derive(Parser, Debug)]
pub struct Args {
    /// The name of the hwmon temp input to use.
    #[arg(short, long)]
    name: String,

    /// The label of the hwmon temp*_label file, if present.
    #[arg(short, long)]
    label: Option<String>,

    /// The link to create for the specified hwmon temp input.
    #[arg(short = 'p', long)]
    link_path: String,
}

fn map_temp_input(entry: DirEntry) -> Option<(String, DirEntry)> {
    if let Some(name) = entry.file_name().to_str() {
        if name.contains(PATTERN_TEMP) && name.contains(PATTERN_INPUT) {
            return Some((name.to_string(), entry));
        }
    }
    None
}

fn force_symlink(original: &Path, link: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_file(link);
    return symlink(original, link).map_err(|e| e.into());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    for temp_input_pair in WalkDir::new(HWMON_DIR)
        .follow_links(true)
        .max_depth(2)
        .into_iter()
        .filter_map(|r| r.ok())
        .filter_map(map_temp_input)
    {
        let path = temp_input_pair.1.path();
        if let Some(dir) = path.parent() {
            let name = fs::read_to_string(dir.join(PATTERN_NAME))?;
            if name.trim() != args.name {
                continue;
            }

            match args.label {
                Some(ref label) => {
                    let label_path = temp_input_pair.0.replace(PATTERN_INPUT, PATTERN_LABEL);
                    if let Ok(hwmon_label) = fs::read_to_string(dir.join(label_path)) {
                        if hwmon_label.trim() == label {
                            return force_symlink(temp_input_pair.1.path(), &args.link_path);
                        }
                    }
                }
                None => {
                    return force_symlink(temp_input_pair.1.path(), &args.link_path);
                }
            }
        }
    }

    Err("no matching hwmon temp input found".into())
}
