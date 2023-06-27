use std::fs::{self};
use std::io::Write;
use std::path::{PathBuf};
extern crate fs_extra;

use clap::Parser;
use fs_extra::{dir, move_items};
use crate::command::{CmdReturn, CommandCtx};
use crate::flags_parse;


#[derive(Parser, Debug)]
struct Args {
    ///-t, --target-directory=DIRECTORY  move all SOURCE arguments into DIRECTORY
    #[arg(short, long)]
    target: Option<String>,

    paths: Vec<String>,
}

// Notice the error handling:
// - If everything goes well, we return exit code as Ok(0)
// - If something is wrong with the user's input, we write to stderr and return Ok(1)
// - If something went wrong with the ctx io, we return an Err

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    let move_options = dir::CopyOptions::new(); 
    let mut from_paths = Vec::new();
    let mut output_path = "".to_string();
    let mut paths = args.paths;   

    if let Some(target) = args.target {
        let temp_path = PathBuf::from(&target);
        if temp_path.is_dir() || temp_path.is_file() {
            output_path = target;
        }
    }

    if paths.is_empty()  {
        writeln!(ctx.stderr, "mv: missing file operand")?;
        writeln!(ctx.stderr, "Try 'mv --help' for more information.")?;

        ctx.stderr.flush()?;

        return Ok(1)
    } else {
        if paths.len() < 2 && output_path.is_empty() {
            writeln!(ctx.stderr, "mv: missing destination file operand after {}", paths.last().unwrap())?;
            writeln!(ctx.stderr, "Try 'mv --help' for more information.")?;
            ctx.stderr.flush()?;
    
            return Ok(1)
        }

        if output_path.is_empty() {
            output_path = paths.pop().unwrap();
        }

        if paths.len() == 1 {
            let temp_path = PathBuf::from(&output_path);
            if !temp_path.is_dir() && !temp_path.is_file() {
                let path = PathBuf::from(paths.last().unwrap());

                if !path.is_dir() && !path.is_file() && !path.exists() {
                    writeln!(ctx.stderr, "mv: cannot stat '{}': No such file or directory", path.display())?;
                    return Ok(1)
                } else {
                    fs::rename(paths.last().unwrap(), &output_path)?;
                    return Ok(0)
                }
            }
        } 

        let temp_path = PathBuf::from(&output_path); 
        if !temp_path.is_dir() { // If output path is not a directory
            writeln!(ctx.stderr, "mv: failed to access '{}': No such file or directory", &output_path)?;

            ctx.stderr.flush()?;
            return Ok(1)
        }

        for path_string in paths.iter() {
            let path = PathBuf::from(path_string);

            if !path.is_dir() && !path.is_file() && !path.exists() {
                writeln!(ctx.stderr, "mv: cannot stat '{}': No such file or directory", path.to_str().unwrap())?;
            } else if &output_path == path_string {
                if path.is_dir() {
                    writeln!(ctx.stderr, "mv: cannot move '{}' to a subdirectory of itself, '{}'", path.to_str().unwrap(), &output_path)?;
                } else {
                    writeln!(ctx.stderr, "mv: cannot stat '{}': No such file or directory", path.to_str().unwrap())?;
                }
            } else {
                from_paths.push(path_string);
            }
        }
    }

    move_items(
        &from_paths,
        &output_path,
        &move_options
    )?;

    ctx.stdout.flush()?;
    ctx.stderr.flush()?;

    Ok(0)
}