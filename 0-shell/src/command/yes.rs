use clap::Parser;

use crate::command::CommandCtx;
use crate::flags_parse;
use std::io::Write;

use super::CmdReturn;

/// be repetitively affirmative
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// String to repeat
    #[arg(default_value = "y")]
    repeat: String,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    loop {
        writeln!(&mut ctx.stdout, "{}", args.repeat)?;
    }
}
