use crate::{command::CommandCtx, flags_parse};
use clap::Parser;
use itertools::Itertools;

use super::CmdReturn;

const LINE_LENGTH: usize = 39;

// Macro that helps auto generate the built in cowfiles list
macro_rules! make_cows {
    (
        $name:ident {
            $(
                $variant:ident {
                    name: $value:literal,
                    prompt: $prompt:literal,
                },
            )*
        }
    ) => {
        #[derive(Debug, Clone, clap::ValueEnum)]
        pub enum $name {
            $( $variant, )*
        }

        impl $name {
            fn name(&self) -> String {
                match self {
                    $( $name::$variant => String::from($value), )*
                }
            }

            fn prompt(&self) -> Vec<String> {
                match self {
                    $( $name::$variant => $prompt.lines().skip(1).map(|l| l.to_string()).collect(), )*
                }
            }

            /// Returns list of all enum cows
            fn cows() -> Vec<String> {
                vec![
                    $( $value.to_string(), )*
                ]
            }
        }

        impl std::fmt::Display for Cow {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for line in self.prompt() {
                    writeln!(f, "{line}")?;
                }

                Ok(())
            }
        }
    }
}

make_cows!(Cow {
    Cow {
        name: "cow",
        prompt: r"
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
",
    },
    Moose {
        name: "moose",
        prompt: r"
\
 \   \_\_    _/_/
  \      \__/
         (oo)\_______
         (__)\       )\/\
             ||----w |
             ||     ||
",
    },
});

fn list_cows(ctx: &mut CommandCtx) -> anyhow::Result<()> {
    writeln!(&mut ctx.stdout, "Built in cowfiles:")?;
    writeln!(&mut ctx.stdout, "{}", Cow::cows().join(", "))?;
    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    /// List defined cows
    #[arg(short)]
    list: bool,

    #[arg(short = 'f', long, default_value = "cow")]
    cowfile: Cow,

    message: Vec<String>,
}

pub fn handle_command(ctx: &mut CommandCtx) -> CmdReturn {
    let args = flags_parse!(Args, ctx);

    if args.list {
        list_cows(ctx)?;
        return Ok(0);
    }

    let message = if args.message.is_empty() {
        let mut pipe_info = String::new();
        ctx.stdin.read_to_string(&mut pipe_info).unwrap();
        pipe_info
    } else {
        args.message.join(" ")
    };

    if message.len() <= LINE_LENGTH {
        let line = message.trim();

        writeln!(&mut ctx.stdout, " {:_<1$}", "", line.len() + 2)?;
        writeln!(&mut ctx.stdout, "< {} >", line)?;
        writeln!(&mut ctx.stdout, " {:-<1$}", "", line.len() + 2)?;
    } else {
        let lines = split_infinitely_at_vec(
            message.split('\n').map(|a| a.to_string()).collect(),
            LINE_LENGTH,
        );

        writeln!(&mut ctx.stdout, " {:_<1$}", "", LINE_LENGTH + 2)?;

        for (i, line) in lines.iter().enumerate() {
            write!(
                &mut ctx.stdout,
                "{}",
                if i == 0 {
                    "/"
                } else if i == lines.len() - 1 {
                    "\\"
                } else {
                    "|"
                }
            )?;

            write!(&mut ctx.stdout, " {:<1$} ", line.trim(), LINE_LENGTH)?;

            writeln!(
                &mut ctx.stdout,
                "{}",
                if i == 0 {
                    "\\"
                } else if i == lines.len() - 1 {
                    "/"
                } else {
                    "|"
                }
            )?;
        }

        writeln!(&mut ctx.stdout, " {:-<1$}", "", LINE_LENGTH + 2)?;
    }

    writeln!(&mut ctx.stdout, "{}", args.cowfile)?;

    Ok(0)
}

fn split_infinitely_at_vec(texts: Vec<String>, split_at: usize) -> Vec<String> {
    texts
        .into_iter()
        .map(|text| split_infinitely_at(text, split_at))
        .concat()
}

fn split_infinitely_at(text: String, split_at: usize) -> Vec<String> {
    if text.len() > split_at {
        let (left, right) = text.split_at(split_at);
        vec![
            split_infinitely_at(left.to_string(), split_at),
            split_infinitely_at(right.to_string(), split_at),
        ]
        .concat()
    } else {
        vec![text]
    }
}
