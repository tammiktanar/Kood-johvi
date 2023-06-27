use clap::Parser;

use crate::error::ExpectedError;
use crate::{command::CommandCtx, constant::HOME, flags_parse};
use anyhow::Context;
use std::env;

use super::CmdReturn;

#[derive(Parser, Debug)]
struct Args {
    directory: Option<String>,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    let path = args.directory.unwrap_or(env::var(HOME)
        .context("error finding home")
        .expected(1)?
    );

    env::set_current_dir(&path)
        .with_context(|| format!("error changing directory to {path}"))
        .expected(1)?;

    Ok(0)
}
