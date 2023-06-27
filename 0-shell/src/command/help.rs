use itertools::Itertools;
use std::io::Write;

use crate::command::CommandCtx;

use super::{CmdReturn, BUILTIN_COMMANDS};

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    writeln!(
        &mut ctx.stdout,
        "built in commands: {}",
        BUILTIN_COMMANDS.keys().join(", ")
    )?;
    Ok(0)
}
