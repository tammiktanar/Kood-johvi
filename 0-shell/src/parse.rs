use std::borrow::Cow;

use nom::branch::alt;
use nom::bytes::streaming::{escaped_transform, is_not, tag, take, take_while};
use nom::character::complete::one_of;
use nom::character::streaming::{char as ch, line_ending, space0};
use nom::combinator::{not, opt, peek, value};
use nom::error::{Error, ErrorKind};
use nom::multi::{fold_many1};
use nom::Parser;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};

use crate::pipeline::{OutTarget, Pipeline, PipelinePart};
use crate::sequence::{Condition, Sequence};

const SPECIAL_CHARS: &str = "><;&|\\'\" \t\r\n";

macro_rules! cow {
    ($x:expr) => {nom::combinator::map($x, std::borrow::Cow::from)};
}

pub fn parse_sequence(input: &str) -> nom::IResult<&str, Sequence> {
    // Test if empty input
    if let Ok((leftover, _)) = preceded(space0::<&str, Error<&str>>, line_ending)(input) {
        return Ok((leftover, Sequence::default()))
    }


    let (input, first) = parse_pipeline(input)?;

    let mut pipelines = vec![(Condition::Always, first)];

    let mut parser = pair(parse_sequence_condition, parse_pipeline);

    let mut input = input;
    loop {
        match parser(input) {
            Ok((leftover, pipeline)) => {
                input = leftover;
                pipelines.push(pipeline);
            }
            Err(nom::Err::Error(_)) => break,
            Err(err) => return Err(err),
        };
    }

    let (input, _) = preceded(space0, line_ending)(input)?;

    let sequence = Sequence {
        pipelines,
        background: false,
    };

    Ok((input, sequence))
}

pub fn parse_sequence_condition(input: &str) -> nom::IResult<&str, Condition> {
    preceded(space0, alt((
        value(Condition::Always, ch(';')),
        value(Condition::And, tag("&&")),
        value(Condition::Or, tag("||")),
    )))(input)
}

pub fn parse_pipeline(mut input: &str) -> nom::IResult<&str, Pipeline> {
    #[rustfmt::skip]
    let parse = alt((
        value(PipelinePart::Pipe(OutTarget::Stdout), delimited(space0, ch('|'), not(peek(ch('|'))))),
        value(PipelinePart::Pipe(OutTarget::Both), delimited(space0, tag("|&"), not(peek(ch('&'))))),
        delimited(space0, separated_pair(one_of("12"), tag(">&"), one_of("12")), peek(one_of(SPECIAL_CHARS))).map(|(from, to): (char, char)| {
            let from = from.to_digit(10).unwrap().try_into().unwrap();
            let to = to.to_digit(10).unwrap().try_into().unwrap();
            PipelinePart::Redirect(from, to)
        }),
        preceded(preceded(space0, tag("&>")), parse_arg).map(|path| PipelinePart::WriteFile(OutTarget::Both, path)),
        separated_pair(terminated(opt(one_of("12")), ch('>')), space0, parse_arg).map(|(digit, path)| {
            PipelinePart::WriteFile(digit.unwrap_or('1').to_digit(10).unwrap().try_into().unwrap(), path)
        }),
        preceded(preceded(space0, tag("&>>")), parse_arg).map(|path| PipelinePart::AppendFile(OutTarget::Both, path)),
        separated_pair(terminated(opt(one_of("12")), tag(">>")), space0, parse_arg).map(|(digit, path)| {
            PipelinePart::AppendFile(digit.unwrap_or('1').to_digit(10).unwrap().try_into().unwrap(), path)
        }),
        preceded(preceded(tag("<"), space0), parse_arg).map(PipelinePart::ReadFile),
        parse_arg.map(PipelinePart::Arg),
    ));

    let mut parse = preceded(space0, parse);

    let mut parts = vec![];
    let mut can_pipe = false;

    loop {
        match parse(input) {
            Ok((leftover, part)) => {
                match part {
                    PipelinePart::Arg(_) => can_pipe = true,
                    PipelinePart::Pipe(_) => {
                        if can_pipe {
                            can_pipe = false;
                        } else {
                            return Err(nom::Err::Error(Error::new(input, ErrorKind::Verify)));
                        }
                    }
                    _ => {}
                }

                input = leftover;
                parts.push(part);
            }
            Err(nom::Err::Error(e)) => {
                if !can_pipe {
                    return Err(nom::Err::Error(e));
                }
                break;
            }
            Err(err) => return Err(err),
        }
    }

    Ok((input, parts))
}


// pub fn parse_args(input: &str) -> nom::IResult<&str, Args> {
//     let parser = preceded(space0, parse_arg);
//     many1(parser)(input)
// }

pub fn parse_arg(input: &str) -> nom::IResult<&str, String> {
    let parser = alt((
        cow!(parse_single_quoted),
        parse_double_quoted,
        parse_normal,
    ));

    fold_many1(parser, String::new, |mut acc, item| {
        acc.push_str(&item);
        acc
    })(input)
}

fn parse_single_quoted(input: &str) -> nom::IResult<&str, &str> {
    delimited(
        ch('\''),
        take_while(|c| c != '\''),
        ch('\''),
    )(input)
}

fn parse_double_quoted(input: &str) -> nom::IResult<&str, Cow<str>> {
    delimited(
        ch('"'),
        alt((
            cow!(value("", peek(ch('"')))),
            cow!(terminated(is_not("\"\\"), not(peek(ch('\\'))))),
            cow!(escaped_transform(is_not("\"\\"), '\\', escaped_str)),
        )),
        ch('"'),
    )(input)
}

fn parse_normal(input: &str) -> nom::IResult<&str, Cow<str>> {
    let mut parser = alt((
        cow!(terminated(is_not(SPECIAL_CHARS), not(peek(ch('\\'))))),
        cow!(escaped_transform(is_not(SPECIAL_CHARS), '\\', escaped_str)),
    ));
    // Make sure we've consumed at least 1 char
    // TODO: Try removing this
    parser(input).and_then(|res| if res.0.len() < input.len() {
        Ok(res)
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Verify)))
    })
}

fn escaped_str(input: &str) -> nom::IResult<&str, &str> {
    alt((
        value("", line_ending),
        // value("\n", tag("n")),
        take(1_usize)
    ))(input)
}


#[cfg(test)]
mod test {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_single_quoted() {
        assert_eq!(parse_single_quoted("'hello there' outside"), Ok((" outside", "hello there")));
        assert_eq!(parse_single_quoted("''"), Ok(("", "")));
        assert_matches!(parse_single_quoted("'one side"), Err(nom::Err::Incomplete(_)));
        assert_matches!(parse_single_quoted("something 'in front'"), Err(nom::Err::Error(_)));
        assert_matches!(parse_single_quoted(" unquoted text! "), Err(nom::Err::Error(_)));
    }

    #[test]
    fn test_double_quoted() {
        assert_eq!(parse_double_quoted("\"hello there\" outside"), Ok((" outside", "hello there".into())));
        assert_eq!(parse_double_quoted("\"hello\\\" there\" outside"), Ok((" outside", "hello\" there".into())));
        assert_eq!(parse_double_quoted("\"hello\\\nthere\" outside"), Ok((" outside", "hellothere".into())));
        assert_eq!(parse_double_quoted("\"\""), Ok(("", "".into())));
        assert_matches!(parse_double_quoted("\"hello\"").unwrap().1, Cow::Borrowed(_));
        assert_matches!(parse_double_quoted("\"hello\\a\"").unwrap().1, Cow::Owned(_));
        assert_matches!(parse_double_quoted("\"one side"), Err(nom::Err::Incomplete(_)));
        assert_matches!(parse_double_quoted("something \"in front\""), Err(nom::Err::Error(_)));
        assert_matches!(parse_double_quoted(" unquoted text! "), Err(nom::Err::Error(_)));
    }

    #[test]
    fn test_normal() {
        assert_eq!(parse_normal("hello\\ there my dude"), Ok((" my dude", "hello there".into())));
        assert_eq!(parse_normal("hello\nthere"), Ok(("\nthere", "hello".into())));
        assert_eq!(parse_normal("hello\\\nthere\n"), Ok(("\n", "hellothere".into())));
        assert_eq!(parse_normal("\\\n\n"), Ok(("\n", "".into())));
        assert_matches!(parse_normal("hello ").unwrap().1, Cow::Borrowed(_));
        assert_matches!(parse_normal("hello\\a ").unwrap().1, Cow::Owned(_));
        assert_matches!(parse_normal(" special char in front"), Err(nom::Err::Error(_)));
    }

    #[test]
    fn test_arg() {
        assert_eq!(parse_arg("hello\\ there my dude"), Ok((" my dude", "hello there".into())));
        assert_eq!(parse_arg("hello\\ 'there my dude'\\\n\" cool beans\"\n"), Ok(("\n", "hello there my dude cool beans".into())));
    }

    // TODO: Add tests for some of the newer functions
    // TODO: Make sure to test that "cmdA | | cmdB" gives an error
}
