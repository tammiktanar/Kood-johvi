use clap::Parser;

use crate::command::{CmdExitCode, CommandCtx};
use crate::{flags_parse, EXIT};

use super::CmdReturn;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value_t = 0)]
    exit_code: CmdExitCode,

    /// When you just want to test exit code behaviour and not actually exit the shell
    #[arg(short, long)]
    dont_exit: bool,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    if !(args.dont_exit || ctx.stdin.is_piped() || ctx.stdout.is_piped() || ctx.stderr.is_piped()) {
        // Only actually exit if we're not part of a pipeline
        // Don't know why, but that's how it works in linux
        EXIT.set(args.exit_code).ok();
    }

    Ok(args.exit_code)
}
