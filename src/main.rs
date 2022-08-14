#![allow(dead_code)]
use std::{path::PathBuf, collections::HashMap};
use std::path::Path;
use std::fs;

use sha256;
use clap::{App, arg, value_parser};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    Error,
    ShowSameFiles,
    ShowDiffFiles,
    MergeIntoA,
    MergeIntoB,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    dir_a: PathBuf,
    dir_b: PathBuf,
    action: Action,
}

impl Default for Args {
    fn default() -> Args {
        Args {
            dir_a: PathBuf::new(),
            dir_b: PathBuf::new(),
            action: Action::Error,
        }
    }
}

impl Args {
    pub fn new() -> Args {
        Args {
            ..Default::default()
        }
    }
}

fn parse_args() -> Args {
    let matches = App::new("merge-tool")
        .version("0.1.0")
        .about("This is my nice merge-tool application")
        .author("me")
        .arg(arg!(-A --dirA <DIR> "Defines directory A").required(true).value_parser(value_parser!(PathBuf)))
        .arg(arg!(-B --dirB <DIR> "Defines directory B").required(true).value_parser(value_parser!(PathBuf)))
        .arg(arg!(--action <ACTION> "Defines the action that should happen.").required(true).help("ACTION: diff | equal | merge_into_a (delete from b) | merge_into_b (delete from a)"))
        .get_matches();

    let mut args = Args::new();

    if let Some(dir_a) = matches.get_one::<PathBuf>("dirA") {
        args.dir_a = dir_a.to_path_buf();
    }
    if let Some(dir_b) = matches.get_one::<PathBuf>("dirB") {
        args.dir_b = dir_b.to_path_buf();
    }
    if let Some(action) = matches.get_one::<String>("action") {
        match action.as_str() {
            "diff" => args.action = Action::ShowDiffFiles,
            "equal" => args.action = Action::ShowSameFiles,
            "merge_into_a" => args.action = Action::MergeIntoA,
            "merge_into_b" => args.action = Action::MergeIntoB,
            _ => {
                eprintln!("Action '{action}' is not valid.\nRerun with --help for more information.");
                std::process::exit(1);
            }
        }
    }
    args
}


fn get_shas_of_files(path: PathBuf) -> std::io::Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for file in fs::read_dir(path)? {
        let fname = file?.path().to_str().unwrap().to_string();
        let file = Path::new(&fname);
        map.insert(sha256::digest_file(file)?, fname.clone());
    }
    Ok(map)
}

fn show_same_files(args: Args) -> std::io::Result<()> {
    let dir_a_map = get_shas_of_files(args.dir_a)?;
    let dir_b_map = get_shas_of_files(args.dir_b)?;

    for (hash, fname) in dir_a_map.iter() {
        if dir_b_map.contains_key(hash) {
            println!("The files '{}' and '{}' are identical", fname, dir_b_map[hash]);
        }
    }

    Ok(())
}

fn show_diff_files(args: Args) -> std::io::Result<()> {
    let dir_a_map = get_shas_of_files(args.dir_a.clone())?;
    let dir_b_map = get_shas_of_files(args.dir_b.clone())?;

    for (hash, fname) in dir_a_map.iter() {
        if !dir_b_map.contains_key(hash) {
            println!("There file '{fname}' is unique in the directories '{}' and '{}'", args.dir_a.display(), args.dir_b.display());
        }
    }
    for (hash, fname) in dir_b_map.iter() {
        if !dir_a_map.contains_key(hash) {
            println!("There file '{fname}' is unique in the directories '{}' and '{}'", args.dir_a.display(), args.dir_b.display());
        }
    }
    Ok(())
}

fn merge_into_a(args: Args) -> std::io::Result<()> {
    let dir_a_map = get_shas_of_files(args.dir_a)?;
    let dir_b_map = get_shas_of_files(args.dir_b)?;

    for (hash, _) in dir_a_map.iter() {
        if dir_b_map.contains_key(hash) {
            let to_delete = &dir_b_map[hash];
            println!("Deleting '{}'.", to_delete);
            fs::remove_file(to_delete)?;
        }
    }
    Ok(())
}

fn merge_into_b(args: Args) -> std::io::Result<()> {
    let dir_a_map = get_shas_of_files(args.dir_a)?;
    let dir_b_map = get_shas_of_files(args.dir_b)?;

    for (hash, _) in dir_b_map.iter() {
        if dir_a_map.contains_key(hash) {
            let to_delete = &dir_a_map[hash];
            println!("Deleting '{}'.", to_delete);
            fs::remove_file(to_delete)?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = parse_args();

    match args.action {
        Action::ShowSameFiles => show_same_files(args)?,
        Action::ShowDiffFiles => show_diff_files(args)?,
        Action::MergeIntoA => merge_into_a(args)?,
        Action::MergeIntoB => merge_into_b(args)?,
        _ => unimplemented!()
    }

    Ok(())
}
