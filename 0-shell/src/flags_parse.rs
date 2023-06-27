/// Accepts your args/flags struct and ctx for printing errors
#[macro_export]
macro_rules! flags_parse {
    ($clap_struct:ty, $ctx:expr) => {
        match <$clap_struct>::try_parse_from(&$ctx.args) {
            Ok(a) => a,
            Err(err) => {
                writeln!(&mut $ctx.stdout, "{}", err.render())?;

                return match err.kind() {
                    clap::error::ErrorKind::DisplayHelp
                    | clap::error::ErrorKind::DisplayVersion => Ok(0),
                    _ => Ok(1),
                };
            }
        }
    };
}
