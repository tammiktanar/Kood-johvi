use crate::command::CommandCtx;
use crate::error::ExpectedError;
use crate::flags_parse;
use anyhow::{bail, Context};
use clap::Parser;
use std::fs;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};

use super::CmdReturn;

/// Remove files or directories
#[derive(Parser, Debug)]
struct Args {
    /// Ignore nonexistent files, never prompt
    #[arg(short, long)]
    force: bool,

    /// Prompt once before removing more than three files, or when removing recursively.
    /// Less intrusive than -i, while still giving protection against most mistakes
    #[arg(short = 'I')]
    once: bool,

    /// Prompt before every removal
    #[arg(short = 'i')]
    always: bool,

    /// Remove directories and their contents recursively
    #[arg(short, short_alias = 'R', long)]
    recursive: bool,

    /// Explain what is being done
    #[arg(short, long)]
    verbose: bool,

    /// Do not treat '/' specially
    #[arg(long)]
    no_preserve_root: bool,

    files: Vec<String>,
}

enum Warn {
    Never,
    Once,
    Always,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    let warn = match (args.force, args.once, args.always) {
        (true, _, _) => Warn::Never,
        (_, true, _) => Warn::Once,
        (_, _, true) => Warn::Always,
        (_, _, _) => Warn::Never,
    };

    if args.files.len() >= 3 && matches!(warn, Warn::Once) {
        let answer = ask(
            &format!("rm: remove {} arguments? ", args.files.len()),
            &mut ctx.stdin,
            &mut ctx.stdout,
        )?;
        if !is_yes(&answer) {
            return Ok(1);
        }
    }

    args.files
        .iter()
        .map(|arg| -> anyhow::Result<_> {
            if !args.no_preserve_root && arg.as_str() == "/" {
                bail!("it is dangerous to operate recursively on '/' \
                (--no-preserve-root to bypass this)")
            }

            let metadata = match fs::metadata(arg) {
                Ok(md) => md,
                Err(err) => match err.kind() {
                    ErrorKind::NotFound if args.force => return Ok(()),
                    _ => bail!("cannot remove {arg:?}: {err:#}"),
                },
            };

            if metadata.is_dir() && !args.recursive {
                bail!("{arg:?} is a dir: use -r flag to delete recursively");
            }

            if metadata.is_dir() {
                fs::remove_dir_all(arg)
            } else {
                fs::remove_file(arg)
            }.with_context(|| format!("cannot remove {arg:?}"))?;

            if args.verbose {
                writeln!(&mut ctx.stdout, "removed {arg:?}")?;
            }

            Ok(())
        })
        .collect::<Result<Vec<_>, _>>()
        .expected(1)?;

    Ok(0)
}

fn ask(question: &str, reader: impl Read, mut writer: impl Write) -> anyhow::Result<String> {
    write!(writer, "{}", question)?;
    let mut buf_reader = BufReader::new(reader);
    let mut buf = String::new();
    buf_reader.read_line(&mut buf)?;
    Ok(buf)
}

fn is_yes(input: &str) -> bool {
    matches!(input.to_ascii_lowercase().as_str(), "y" | "yes")
}
