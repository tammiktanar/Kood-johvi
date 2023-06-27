use clap::Parser;
use std::{fs, io::Write};

use crate::command::CommandCtx;
use crate::flags_parse;

use super::CmdReturn;

#[derive(Parser, Debug)]
struct Args {
    /// no error if existing, make parent directories as needed
    #[arg(short, long)]
    parents: bool,

    paths: Vec<String>,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    let mut has_failed = false;
    for folder in args.paths {
        let res = if args.parents {
            fs::create_dir_all(&folder)
        } else {
            fs::create_dir(&folder)
        };

        if let Err(err) = res {
            has_failed = true;
            writeln!(
                &mut ctx.stderr,
                "mkdir: error creating folder {:?}: {}",
                folder, err
            )?;
        };
    }

    Ok(if has_failed { 1 } else { 0 })
}
