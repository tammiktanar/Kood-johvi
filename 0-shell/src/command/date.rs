use crate::{command::CommandCtx, flags_parse};
use chrono::Utc;
use clap::Parser;
use std::io::Write;

use super::CmdReturn;

#[derive(Parser, Debug)]
struct Args {
    format: Option<String>,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    match args.format {
        Some(f) => writeln!(&mut ctx.stdout, "{}", Utc::now().format(&f))?,
        None => writeln!(&mut ctx.stdout, "{}", Utc::now())?,
    }

    Ok(0)
}
