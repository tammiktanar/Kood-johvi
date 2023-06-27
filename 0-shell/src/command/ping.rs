use super::CmdReturn;
use crate::command::CommandCtx;

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    writeln!(ctx.stdout, "pong")?;
    Ok(0)
}
