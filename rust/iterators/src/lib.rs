pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(v: u64) -> Self {
        Self { v }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }

        self.v = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v.checked_mul(3)?.checked_add(1)?
        };

        Some(self.v)
    }
}

pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut collatz = Collatz{ v: n };
    let mut count = 0;

    loop {
        if n == 1 {
          break
        };

        n = collatz.next()?;
        count += 1;
    };

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", collatz(0).unwrap());
    }
}
