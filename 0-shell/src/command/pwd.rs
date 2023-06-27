use super::CmdReturn;
use crate::command::CommandCtx;
use std::{env, io::Write};

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    writeln!(ctx.stdout, "{}", env::current_dir()?.display())?;
    Ok(0)
}
