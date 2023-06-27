#![allow(dead_code)]

use std::env;
use std::io;
use std::process::ExitCode;

use anyhow::anyhow;
use once_cell::sync::OnceCell;

use crate::parse::{parse_sequence};
use crate::sequence::{handle_sequence, Sequence};

mod command;
mod parse;
mod pipeline;
mod constant;
mod traits;
mod sequence;
mod flags_parse;
mod error;

static EXIT: OnceCell<u8> = OnceCell::new();

fn main() -> ExitCode {
    let mut buf = String::new();

    loop {
        if let Some(&exit_code) = EXIT.get() {
            return ExitCode::from(exit_code);
        }

        let sequence = match read_sequence(&mut buf) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("{:#}", err);
                continue;
            }
        };

        // eprintln!("{:?}", sequence);

        handle_sequence(sequence)
            .unwrap_or_else(handle_internal_error);
    }
}

fn read_sequence(buf: &mut String) -> anyhow::Result<Sequence> {
    eprint!("{}", shell_pretext());
    let (leftover, sequence) = loop {
        io::stdin().read_line(buf)?;
        match parse_sequence(buf.as_str()) {
            Ok(res) => break res,
            Err(nom::Err::Incomplete(_)) => {
                eprint!("> ");
                continue;
            }
            Err(err) => {
                let err = Err(anyhow!("shell parsing error: {}", err));
                buf.clear();
                return err;
            }
        };
    };

    // Make sure we're consuming the input to avoid an infinite loop
    assert!(leftover.len() < buf.len(), "leftover: \n{}, buf: \n{}", leftover, buf);

    *buf = leftover.to_string();

    Ok(sequence)
}

fn shell_pretext() -> String {
    let user = env::var(constant::USER).unwrap();
    let dir = env::current_dir().unwrap();
    format!("{} 0-shell {}\n$ ", user, dir.display())
}

fn handle_internal_error(err: anyhow::Error) {
    eprintln!("{:#}", err.context("shell internal error"));
}