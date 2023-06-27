use itertools::Itertools;
use std::io::Write;

use crate::command::CommandCtx;

use super::CmdReturn;

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    writeln!(&mut ctx.stdout, "{}", ctx.args.iter().skip(1).join(" "))?;
    Ok(0)
}
