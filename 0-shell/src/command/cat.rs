use clap::Parser;

use crate::command::CommandCtx;
use crate::error::ExpectedError;
use crate::flags_parse;
use anyhow::Context;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, Write};

use super::CmdReturn;

// Notice the error handling:
// - If everything goes well, we return exit code as Ok(0)
// - If something is wrong with the user's input, we write to stderr and return Ok(1)
// - If something went wrong with the ctx io, we return an Err

/// Concatenate files to standard output.
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// number all output lines
    #[arg(short, long)]
    number: bool,

    /// Files to concatenate
    files: Vec<String>,
}

fn numbered_copy<R: Read>(reader: R, mut writer: impl Write) -> Result<(), io::Error> {
    for (i, v) in BufReader::new(reader).lines().enumerate() {
        writeln!(writer, "{:^4} {}", i + 1, v?)?;
    }

    Ok(())
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    if args.files.is_empty() {
        if args.number {
            numbered_copy(&mut ctx.stdin, &mut ctx.stdout)?;
        } else {
            io::copy(&mut ctx.stdin, &mut ctx.stdout)?;
        }
    } else {
        for filename in args.files {
            let mut file = File::open(&filename)
                .with_context(|| format!("error opening file {filename:?}"))
                .expected(1)?;

            if args.number {
                numbered_copy(&mut file, &mut ctx.stdout)
                    .context(filename.clone())
                    .expected(1)?;
            } else {
                io::copy(&mut file, &mut ctx.stdout)
                    .context(filename.clone())
                    .expected(1)?;
            }
        }
    }

    Ok(0)
}
