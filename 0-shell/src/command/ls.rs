use crate::{command::CommandCtx, flags_parse};
use chrono::{DateTime, Local};
use clap::Parser;
use std::{
    env,
    fs::{self, Metadata},
    io::Write,
};

use crate::error::ExpectedError;
use anyhow::Context;
#[cfg(unix)]
use std::os::unix::prelude::{FileTypeExt, MetadataExt, PermissionsExt};
#[cfg(windows)]
use std::os::windows::prelude::MetadataExt;

use super::CmdReturn;

#[derive(Parser, Debug)]
struct Args {
    /// Show hidden and 'dot' files
    #[arg(short, long)]
    all: bool,

    /// Display a slash (‘/’) immediately after each pathname that is a directory,
    /// an asterisk (‘*’) after each that is executable,
    /// an at sign (‘@’) after each symbolic link,
    /// an equals sign (‘=’) after each socket,
    /// a percent sign (‘%’) after each whiteout,
    /// and a vertical bar (‘|’) after each that is a FIFO.
    #[arg(short = 'F', long)]
    classify: bool,

    /// List files in the long format
    #[arg(short, long)]
    long: bool,

    paths: Vec<String>,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    if args.paths.is_empty() {
        match env::current_dir() {
            Ok(dir) => {
                let path = dir.to_str().unwrap();
                let path_metadata = fs::metadata(&dir)
                    .with_context(|| dir.to_string_lossy().to_string())
                    .expected(1)?;

                if args.long && path_metadata.is_dir() {
                    total(ctx, path)?;
                }

                print_files(ctx, &args, path, &path_metadata)?
            }
            Err(err) => {
                return Err(err)
                    .context("Error finding current directory")
                    .expected(1);
            }
        }
    } else if args.paths.len() == 1 {
        let path_metadata = fs::metadata(&args.paths[0])
            .context(args.paths[0].clone())
            .expected(1)?;

        if args.long && path_metadata.is_dir() {
            total(ctx, &args.paths[0])?;
        }

        print_files(ctx, &args, &args.paths[0], &path_metadata)?;
    } else {
        for (i, path) in args.paths.iter().enumerate() {
            let path_metadata = match fs::metadata(path) {
                Ok(p) => p,
                Err(err) => {
                    writeln!(&mut ctx.stderr, "{:?}: {err}", path)?;
                    continue;
                }
            };

            if path_metadata.is_dir() {
                writeln!(ctx.stdout, "{path}:")?;

                if args.long {
                    total(ctx, path)?;
                }
            }

            print_files(ctx, &args, path, &path_metadata)?;

            if args.paths.len() - 1 != i && path_metadata.is_dir() {
                writeln!(ctx.stdout)?;
            }
        }
    }

    ctx.stdout.flush()?;
    Ok(0)
}

fn total(ctx: &mut CommandCtx, dir: &str) -> anyhow::Result<()> {
    let total: u64 = fs::read_dir(dir)
        .unwrap()
        .map(|path| {
            #[cfg(unix)]
            {
                path.unwrap().metadata().unwrap().blocks()
            }
            #[cfg(windows)]
            {
                path.unwrap().metadata().unwrap().file_size()
            }
        })
        .sum();

    writeln!(&mut ctx.stdout, "total {total}")?;
    Ok(())
}

enum FileType {
    RegularFile,
    Directory,
    CharacterFile,
    BlockFile,
    SymbolicLink,
    Fifo,
    Socket,
    Whiteout,
    Executable,
}

/// Unix whiteout permission
const S_IFWHT: u32 = 0o160000;

impl FileType {
    fn to_long_character(&self) -> char {
        match self {
            FileType::RegularFile | FileType::Executable => '-',
            FileType::Directory => 'd',
            FileType::CharacterFile => 'c',
            FileType::BlockFile => 'b',
            FileType::SymbolicLink => 'l',
            FileType::Fifo => 'p',
            FileType::Socket => 's',
            FileType::Whiteout => 'w',
        }
    }

    fn to_classify_character(&self) -> Option<char> {
        match self {
            FileType::Directory => Some('/'),
            FileType::SymbolicLink => Some('@'),
            FileType::Fifo => Some('|'),
            FileType::Socket => Some('='),
            FileType::Whiteout => Some('%'),
            FileType::Executable => Some('*'),
            _ => None,
        }
    }

    fn from_metadata(metadata: &Metadata) -> FileType {
        if metadata.is_dir() {
            FileType::Directory
        } else if metadata.is_symlink() {
            FileType::SymbolicLink
        } else if metadata.is_file() {
            #[cfg(unix)]
            if metadata.permissions().mode() & 0o111 != 0 {
                FileType::Executable
            } else if metadata.file_type().is_socket() {
                FileType::Socket
            } else if metadata.file_type().is_fifo() {
                FileType::Fifo
            } else if metadata.permissions().mode() & S_IFWHT == S_IFWHT {
                // untested but it should work correctly
                FileType::Whiteout
            } else {
                FileType::RegularFile
            }
            #[cfg(windows)]
            {
                FileType::RegularFile
            }
        } else {
            FileType::RegularFile
        }
    }
}

const S_ISUID: u32 = 0o4000;
const S_ISGID: u32 = 0o2000;
const S_ISVTX: u32 = 0o1000;

/// Owner permissions
const S_IRUSR: u32 = 0o400;
const S_IWUSR: u32 = 0o200;
const S_IXUSR: u32 = 0o100;

/// Group permissions
const S_IRGRP: u32 = 0o40;
const S_IWGRP: u32 = 0o20;
const S_IXGRP: u32 = 0o10;

/// Other permissions
const S_IROTH: u32 = 0o4;
const S_IWOTH: u32 = 0o2;
const S_IXOTH: u32 = 0o1;

// TODO: show files in proper order
fn print_files(
    ctx: &mut CommandCtx,
    args: &Args,
    dir: &str,
    dir_metadata: &Metadata,
) -> anyhow::Result<()> {
    let files: Vec<(Metadata, String)> = if !dir_metadata.is_dir() {
        vec![(dir_metadata.clone(), dir.to_string())]
    } else {
        fs::read_dir(dir)
            .unwrap()
            .map(|p| {
                let direntry = p.unwrap();
                (
                    direntry.metadata().unwrap(),
                    direntry.file_name().to_str().unwrap().to_string(),
                )
            })
            .chain(if args.all {
                vec![
                    (fs::metadata(".").unwrap(), String::from(".")),
                    (fs::metadata("..").unwrap(), String::from("..")),
                ]
            } else {
                Vec::new()
            })
            .collect()
    };

    for (metadata, file_name) in files {
        if dir_metadata.is_dir() && file_name.starts_with('.') && !args.all {
            continue;
        }

        let processed_file_name = (|| -> Option<String> {
            args.classify.then_some(format!(
                "{file_name}{}",
                FileType::from_metadata(&metadata).to_classify_character()?
            ))
        })()
        .unwrap_or(file_name.to_string());

        if args.long {
            // Cross-platform stuff

            let file_type = FileType::from_metadata(&metadata).to_long_character();

            let modification_time = DateTime::<Local>::from(metadata.modified().unwrap())
                .format("%d/%m/%Y %H:%M")
                .to_string();

            #[cfg(unix)]
            {
                let mode = metadata.permissions().mode();
                let owner_permission: String = {
                    let readable = if mode & S_IRUSR != 0 { "r" } else { "-" };

                    let writable = if mode & S_IWUSR != 0 { "w" } else { "-" };

                    let other = match (mode & S_IXUSR, mode & S_ISUID) {
                        (0, 0) => "-",
                        (0, _) => "S",
                        (_, 0) => "x",
                        (_, _) => "s",
                    };

                    format!("{readable}{writable}{other}")
                };
                let group_permission: String = {
                    let readable = if mode & S_IRGRP != 0 { "r" } else { "-" };

                    let writable = if mode & S_IWGRP != 0 { "w" } else { "-" };

                    let other = match (mode & S_IXGRP, mode & S_ISGID) {
                        (0, 0) => "-",
                        (0, _) => "S",
                        (_, 0) => "x",
                        (_, _) => "s",
                    };

                    format!("{readable}{writable}{other}")
                };
                let other_permission: String = {
                    let readable = if mode & S_IROTH != 0 { "r" } else { "-" };

                    let writable = if mode & S_IWOTH != 0 { "w" } else { "-" };

                    let other = match (mode & S_ISVTX, mode & S_IXOTH) {
                        (0, 0) => "-",
                        (0, _) => "x",
                        (_, 0) => "T",
                        (_, _) => "t",
                    };

                    format!("{readable}{writable}{other}")
                };

                #[cfg(windows)]
                let (owner_permission, group_permission, other_permission) = ("", "", "");

                // TODO: give @ or +
                let extended_attributes = "";

                let number_of_links = metadata.nlink();

                // TODO: find out user and group names
                let owner_name = metadata.uid();
                let group_name = metadata.gid();

                let file_size = metadata.size();

                // TODO: padding/align items properly
                writeln!(&mut ctx.stdout, "{file_type}{owner_permission}{group_permission}{other_permission}{extended_attributes} \
                {number_of_links} {owner_name}  {group_name} {file_size} {modification_time} {processed_file_name}")?;
            }

            #[cfg(windows)]
            {
                let file_size = metadata.file_size();

                writeln!(&mut ctx.stdout, "{file_type}rw-r--r-- 0 ? ? {file_size} {modification_time} {processed_file_name}")?;
            }
        } else {
            writeln!(&mut ctx.stdout, "{processed_file_name}")?;
        }
    }

    Ok(())
}
