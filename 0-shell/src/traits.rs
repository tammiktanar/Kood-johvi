use std::fs::File;
use std::io;
use std::io::{Empty, Sink, Stderr, Stdin, Stdout};
use std::process::Stdio;

use os_pipe::{PipeReader, PipeWriter};

use crate::command::{PipeRead, PipeWrite};

pub trait IntoStdio {
    fn into_stdio(self: Box<Self>) -> Stdio;

    fn is_piped(&self) -> bool {
        true
    }
}

impl IntoStdio for File {
    fn into_stdio(self: Box<Self>) -> Stdio {
        (*self).into()
    }
}

impl IntoStdio for PipeReader {
    fn into_stdio(self: Box<Self>) -> Stdio {
        (*self).into()
    }
}

impl IntoStdio for PipeWriter {
    fn into_stdio(self: Box<Self>) -> Stdio {
        (*self).into()
    }
}

impl IntoStdio for Empty {
    fn into_stdio(self: Box<Self>) -> Stdio {
        Stdio::null()
    }
}

impl IntoStdio for Sink {
    fn into_stdio(self: Box<Self>) -> Stdio {
        Stdio::null()
    }
}

impl IntoStdio for Stdin {
    fn into_stdio(self: Box<Self>) -> Stdio {
        Stdio::inherit()
    }

    fn is_piped(&self) -> bool {
        false
    }
}

impl IntoStdio for Stdout {
    fn into_stdio(self: Box<Self>) -> Stdio {
        Stdio::inherit()
    }

    fn is_piped(&self) -> bool {
        false
    }
}

impl IntoStdio for Stderr {
    fn into_stdio(self: Box<Self>) -> Stdio {
        Stdio::inherit()
    }

    fn is_piped(&self) -> bool {
        false
    }
}


pub trait TryCloneReader {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeRead>>;
}

impl TryCloneReader for File {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeRead>> {
        Ok(Box::new(self.try_clone()?))
    }
}

impl TryCloneReader for PipeReader {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeRead>> {
        Ok(Box::new(self.try_clone()?))
    }
}

impl TryCloneReader for Empty {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeRead>> {
        Ok(Box::new(io::empty()))
    }
}

impl TryCloneReader for Stdin {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeRead>> {
        Ok(Box::new(io::stdin()))
    }
}

pub trait TryCloneWriter {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeWrite>>;
}


impl TryCloneWriter for File {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeWrite>> {
        Ok(Box::new(self.try_clone()?))
    }
}

impl TryCloneWriter for PipeWriter {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeWrite>> {
        Ok(Box::new(self.try_clone()?))
    }
}

impl TryCloneWriter for Sink {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeWrite>> {
        Ok(Box::new(io::sink()))
    }
}

impl TryCloneWriter for Stdout {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeWrite>> {
        Ok(Box::new(io::stdout()))
    }
}

impl TryCloneWriter for Stderr {
    fn try_clone(&self) -> anyhow::Result<Box<dyn PipeWrite>> {
        Ok(Box::new(io::stderr()))
    }
}
