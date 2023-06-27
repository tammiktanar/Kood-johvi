use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    // expected public fields
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", l_h.chars().nth(0).unwrap()),
            long_hand: format!("--{}", l_h),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        let f = self.flags.get(&flag).unwrap();
        f(argv[0], argv[1]).unwrap_or_else(|err| err.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f32 = a.parse()?;
    let b: f32 = b.parse()?;
    let res = a / b;
    Ok(res.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f32 = a.parse()?;
    let b: f32 = b.parse()?;
    let res = a % b;
    Ok(res.to_string())
}
