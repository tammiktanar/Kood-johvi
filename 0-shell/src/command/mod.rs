use crate::error::CmdError;
use crate::traits::{IntoStdio, TryCloneReader, TryCloneWriter};
use anyhow::{bail, Context, Result};
use std::io::{Read, Write};
use std::thread::JoinHandle;
use std::{io, thread};

mod cat;
mod cd;
mod cowsay;
mod date;
mod default;
mod echo;
mod exit;
mod help;
mod ls;
mod mkdir;
mod ping;
mod pwd;
mod yes;
mod rm;
mod mv;
mod cp;

pub type CmdReturn = Result<CmdExitCode, CmdError>;
pub type CmdFn = fn(&mut CommandCtx) -> CmdReturn;

static BUILTIN_COMMANDS: phf::Map<&'static str, CmdFn> = phf::phf_map! {
    "ping" => ping::handle_command,
    "cat" => cat::handle_command,
    "echo" => echo::handle_command,
    "exit" => exit::handle_command,
    "mkdir" => mkdir::handle_command,
    "pwd" => pwd::handle_command,
    "cd" => cd::handle_command,
    "ls" => ls::handle_command,
    "cp" => cp::handle_command,
    "mv" => mv::handle_command,
    "date" => date::handle_command,
    "cowsay" => cowsay::handle_command,
    "help" => help::handle_command,
    "yes" => yes::handle_command,
    "rm" => rm::handle_command,
};

pub type Args = Vec<String>;

pub type CmdExitCode = u8;

pub struct CommandCtx {
    pub args: Args,
    pub stdin: Box<dyn PipeRead>,
    pub stdout: Box<dyn PipeWrite>,
    pub stderr: Box<dyn PipeWrite>,
}

pub fn handle_command(mut ctx: CommandCtx) -> Result<JoinHandle<Result<CmdExitCode>>> {
    let cmd = ctx
        .args
        .get(0)
        .expect("handle_command: args[0] doesn't exist?!")
        .clone();

    let command = BUILTIN_COMMANDS
        .get(cmd.as_str())
        // TODO: Replace with default command that tries running a process::Command
        .with_context(|| format!("unknown command: {:?}", cmd))?;

    Ok(thread::spawn(move || {
        let res = match command(&mut ctx) {
            Ok(exit_code) => Ok(exit_code),
            Err(CmdError::Expected(exit_code, err)) => {
                writeln!(ctx.stdout, "{cmd}: {:#}", err)
                    .context("unexpected error while writing expected error")
                    .context(cmd)
                    .map(|_| exit_code)
            }
            Err(CmdError::Unexpected(err)) => {
                Err(err.context("unexpected error").context(cmd))
            },
        };

        ctx.stdout.flush().ok();
        ctx.stderr.flush().ok();

        res
    }))
}

pub trait PipeRead: Read + IntoStdio + TryCloneReader + Send {}

pub trait PipeWrite: Write + IntoStdio + TryCloneWriter + Send {}

impl<T: Read + IntoStdio + TryCloneReader + Send> PipeRead for T {}

impl<T: Write + IntoStdio + TryCloneWriter + Send> PipeWrite for T {}

pub struct CommandCtxBuilder {
    pub args: Args,
    pub stdin: Box<dyn PipeRead>,
    pub stdout: Option<Box<dyn PipeWrite>>,
    pub stderr: Option<Box<dyn PipeWrite>>,
}

impl CommandCtxBuilder {
    pub fn new(stdin: Box<dyn PipeRead>) -> Self {
        Self {
            args: vec![],
            stdin,
            stdout: None,
            stderr: Some(Box::new(io::stderr())),
        }
    }

    pub fn finalize(self, default_out: Box<dyn PipeWrite>) -> Result<CommandCtx> {
        if self.args.is_empty() {
            bail!("tried to finalize a command with empty args");
        }

        Ok(CommandCtx {
            args: self.args,
            stdin: self.stdin,
            stdout: match self.stdout {
                Some(x) => x,
                None => default_out.try_clone()?,
            },
            stderr: match self.stderr {
                Some(x) => x,
                None => default_out.try_clone()?,
            },
        })
    }

    pub fn push_arg(&mut self, arg: String) {
        self.args.push(arg)
    }

    pub fn merge_stderr_to_stdout(&mut self) -> Result<()> {
        self.stderr = match &self.stdout {
            None => None,
            Some(stdout) => Some(stdout.try_clone()?),
        };
        Ok(())
    }

    pub fn merge_stdout_to_stderr(&mut self) -> Result<()> {
        self.stdout = match &self.stderr {
            None => None,
            Some(stdout) => Some(stdout.try_clone()?),
        };
        Ok(())
    }
}
