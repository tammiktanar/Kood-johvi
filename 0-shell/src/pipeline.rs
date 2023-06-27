use std::{io, mem};
use std::fs::File;
use anyhow::bail;
use itertools::Itertools;

use crate::command::{CmdExitCode, CommandCtx, CommandCtxBuilder, handle_command, PipeWrite};
use crate::handle_internal_error;

pub type Pipeline = Vec<PipelinePart>;

#[derive(Debug, Clone, PartialEq)]
pub enum PipelinePart {
    Arg(String), // cmd arg1 arg2
    Pipe(OutTarget), // |
    Redirect(OutTarget, OutTarget), // 2>&1
    WriteFile(OutTarget, String), // > filename
    AppendFile(OutTarget, String), // >> filename
    ReadFile(String), // < filename
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutTarget {
    Stdout,
    Stderr,
    Both,
}

impl OutTarget {
    fn map_ctx(self, ctx: &mut CommandCtxBuilder, mut f: impl FnMut(&mut Option<Box<dyn PipeWrite>>) -> anyhow::Result<()>) -> anyhow::Result<()> {
        match self {
            OutTarget::Stdout => f(&mut ctx.stdout)?,
            OutTarget::Stderr => f(&mut ctx.stderr)?,
            OutTarget::Both => {
                f(&mut ctx.stdout)?;
                f(&mut ctx.stderr)?;
            }
        }
        Ok(())
    }
}

impl TryFrom<u32> for OutTarget {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Stdout),
            2 => Ok(Self::Stderr),
            _ => Err("tried to create an OutTarget from a number not 1 or 2"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PipelineType {
    Foreground,
    Background,
}

pub fn handle_pipeline(pipeline: Pipeline) -> anyhow::Result<CmdExitCode> {
    let contexts = build_contexts(pipeline)?;

    let handles = contexts.into_iter()
        .map(handle_command)
        .collect_vec();

    let results = handles.into_iter()
        .map(|handle| {
            // handle?.join().map_err(|_| anyhow!("panic"))?
            handle?.join().unwrap_or(Ok(1))
        })
        .collect_vec();

    let last_exit_code = *results.last()
        .unwrap_or(&Ok(0))
        .as_ref()
        .unwrap_or(&1);

    results.into_iter()
        .filter_map(|res| res.err())
        .for_each(handle_internal_error);

    Ok(last_exit_code)
}

fn build_contexts(pipeline: Pipeline) -> anyhow::Result<Vec<CommandCtx>> {
    if pipeline.is_empty() {
        return Ok(vec![])
    }

    let mut builder = CommandCtxBuilder::new(Box::new(io::stdin()));
    let mut contexts = vec![];

    for component in pipeline {
        match component {
            PipelinePart::Arg(arg) => {
                builder.push_arg(arg);
            }
            PipelinePart::Pipe(target) => {
                if let OutTarget::Both = target {
                    builder.merge_stderr_to_stdout()?;
                };

                let (reader, writer) = os_pipe::pipe()?;

                let finished_builder = mem::replace(
                    &mut builder,
                    CommandCtxBuilder::new(Box::new(reader)),
                );

                let ctx = finished_builder.finalize(Box::new(writer))?;
                contexts.push(ctx);
            }
            PipelinePart::Redirect(OutTarget::Stderr, OutTarget::Stdout) => builder.merge_stderr_to_stdout()?,
            PipelinePart::Redirect(OutTarget::Stdout, OutTarget::Stderr) => builder.merge_stdout_to_stderr()?,
            PipelinePart::WriteFile(source, path) => {
                let file = File::create(path)?;
                source.map_ctx(&mut builder, |source| {
                    *source = Some(Box::new(file.try_clone()?));
                    Ok(())
                })?;
            }
            PipelinePart::AppendFile(source, path) => {
                let file = File::options().append(true).create(true).open(path)?;
                source.map_ctx(&mut builder, |source| {
                    *source = Some(Box::new(file.try_clone()?));
                    Ok(())
                })?;
            }
            PipelinePart::ReadFile(path) => {
                let file = File::open(path)?;
                builder.stdin = Box::new(file);
            }
            part => bail!("unexpected pipeline part: {:?}", part),
        }
    }

    let ctx = builder.finalize(Box::new(io::stdout()))?;
    contexts.push(ctx);

    Ok(contexts)
}