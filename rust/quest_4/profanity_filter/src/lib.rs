#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Message {
    content: String,
    user: String,
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
    Self{content: ms, user: u}
  }
  pub fn send_ms(&self) -> Option<&str> {
    if self.content.is_empty() || self.content.contains("stupid") {
        return None
    }
    Some(&self.content)
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        None => {return (false, "ERROR: illegal")},
        Some(content) => {return(true, content)},
    }
}