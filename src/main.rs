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
    Merge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    dir_a: PathBuf,
    dir_b: PathBuf,
    merge_dir: PathBuf,
    action: Action,
    with_check: bool,
}

impl Default for Args {
    fn default() -> Args {
        Args {
            dir_a: PathBuf::new(),
            dir_b: PathBuf::new(),
            merge_dir: PathBuf::new(),
            action: Action::Error,
            with_check: false,
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
        .about("This is my nice merge-tool application.")
        .author("me")
        .arg(arg!(-A --dirA <DIR> "Defines directory A.").required(true).value_parser(value_parser!(PathBuf)))
        .arg(arg!(-B --dirB <DIR> "Defines directory B.").required(true).value_parser(value_parser!(PathBuf)))
        .arg(arg!(--action <ACTION> "Defines the action that should happen.").required(true).help("ACTION: diff | equal | merge_into_a (delete from b) | merge_into_b (delete from a) | merge (--merge argument has to be provided)"))
        .arg(arg!(-c --confirmation <BOOL> "Asking for confirmation before deleting a file.").help("BOOL: true | false").required(false).value_parser(value_parser!(bool)))
        .arg(arg!(-m --merge <DIR> "Merge A and B into DIR and DELETES content from A and B").required(false).value_parser(value_parser!(PathBuf)))
        .get_matches();

    let mut args = Args::new();

    if let Some(dir_a) = matches.get_one::<PathBuf>("dirA") {
        args.dir_a = dir_a.to_path_buf();
    }
    if let Some(dir_b) = matches.get_one::<PathBuf>("dirB") {
        args.dir_b = dir_b.to_path_buf();
    }
    if let Some(confirmation) = matches.get_one::<bool>("confirmation") {
        args.with_check = confirmation.clone();
    }

    if let Some(merge_directory) = matches.get_one::<PathBuf>("merge") {
        args.merge_dir = merge_directory.to_path_buf();
    }

    if let Some(action) = matches.get_one::<String>("action") {
        match action.as_str() {
            "diff" => args.action = Action::ShowDiffFiles,
            "equal" => args.action = Action::ShowSameFiles,
            "merge_into_a" => args.action = Action::MergeIntoA,
            "merge_into_b" => args.action = Action::MergeIntoB,
            "merge" => args.action = Action::Merge,
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
            if args.with_check { 
                println!("Remove file '{to_delete}' [y/n]");
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input)?;
                if user_input.starts_with('y') || user_input.starts_with('Y') {
                    println!("Deleting '{}'.", to_delete);
                    fs::remove_file(to_delete)?;
                }
            } else {
                println!("Deleting '{}'.", to_delete);
                fs::remove_file(to_delete)?;
            }
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
            if args.with_check { 
                println!("Remove file '{to_delete}' [y/n]");
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input)?;
                if user_input.starts_with('y') || user_input.starts_with('Y') {
                    println!("Deleting '{}'.", to_delete);
                    fs::remove_file(to_delete)?;
                }
            } else {
                println!("Deleting '{}'.", to_delete);
                fs::remove_file(to_delete)?;
            }
        }
    }
    Ok(())
}

fn merge(args: Args) -> std::io::Result<()> {
    if args.merge_dir.as_os_str().is_empty() {
        eprintln!("You cannot merge without naming an output directory.\nRerun with --help for more information");
        std::process::exit(1);
    }

    if args.merge_dir.exists() {
        println!("The directory '{}' already exists, should it be overwritten [y/n]", args.merge_dir.display());
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;
        if user_input.starts_with('n') || user_input.starts_with('N') {
            std::process::exit(1);
        }
    } else {
        fs::create_dir(&args.merge_dir).expect("XXX");
    }

    let dir_a_map = get_shas_of_files(args.dir_a)?;
    let dir_b_map = get_shas_of_files(args.dir_b)?;

    // copy everything from dir A into new dir
    for (_, fname) in dir_a_map.iter() {
        let file = Path::new(fname);
        let new_name = file.file_name().unwrap();
        let new_name = args.merge_dir.to_str().unwrap().to_string() + "/" + &new_name.to_str().unwrap();
        println!("Moving '{}' to '{}'", fname, new_name);
        fs::rename(fname, new_name)?;
    }

    // copy everything that is not in dir A into new dir
    for (hash, fname) in dir_b_map.iter() {
        if ! dir_a_map.contains_key(hash) {
            let file = Path::new(fname);
            let new_name = file.file_name().unwrap();
            let new_name = args.merge_dir.to_str().unwrap().to_string() + "/" + &new_name.to_str().unwrap();
            println!("Moving '{}' to '{}'", fname, new_name);
            fs::rename(fname, new_name)?;
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
        Action::Merge => merge(args)?,
        _ => unimplemented!()
    }

    Ok(())
}
